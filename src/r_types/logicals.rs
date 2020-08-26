use crate::interface_traits::*;
use crate::internals::{
    c::R_NaLogical,
    sexpr::{SexpType, Sexpr},
};
use crate::r_types::Rbox;

pub struct LogicalVector {
    sexp: Sexpr,
}
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Rlogical {
    False = 0,
    True = 1,
    Na = R_NaLogical,
}

impl Na for Rlogical {
    fn na() -> Self {
        Self::Na
    }
    fn is_na(&self) -> bool {
        match self {
            Self::Na => true,
            _ => false,
        }
    }
}

impl Rlogical {
    pub unsafe fn as_i32_unchecked(self) -> i32 {
        self as i32
    }
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Self::False => Some(false),
            Self::True => Some(true),
            Self::Na => None,
        }
    }
    pub fn from_bool(input: Option<bool>) -> Self {
        match input {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Na,
        }
    }
}

impl LogicalVector {
    pub fn new_rboxed_with_size(size: isize) -> Rbox<Self> {
        Rbox::<Self>::new_with_size(size)
    }
    pub fn as_slice(&self) -> &[Rlogical] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_mut_slice(&mut self) -> &mut [Rlogical] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
    pub fn as_raw_slice(&self) -> &[i32] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_raw_mut_slice(&mut self) -> &mut [i32] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
}

impl TryFromR for &LogicalVector {
    type Error = &'static str;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let ref_expr = unsafe { r.as_mut() }.ok_or("Argument is a null pointer")?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Lgl => Ok(unsafe { &*(ref_expr as *mut Sexpr as *mut LogicalVector) }),
                _ => Err("Argument is not a Logical vector"),
            }
        } else {
            Err("Argument is alt rep")
        }
    }
}

impl TryFromR for &[Rlogical] {
    type Error = <&'static LogicalVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let nv = <&LogicalVector>::try_from_r(r)?;
        Ok(nv.as_slice())
    }
}

impl TryFromR for Rlogical {
    type Error = <&'static LogicalVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let slice = <&[Rlogical]>::try_from_r(r)?;
        match slice.len() {
            1 => Ok(slice[0]),
            _x => Err("Was expecting a Logical vector of size 1"),
        }
    }
}
impl TryFromR for Option<bool> {
    type Error = <&'static LogicalVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        Ok(<Rlogical as TryFromR>::try_from_r(r)?.as_bool())
    }
}

impl IntoR for &LogicalVector {
    fn into_r(self) -> *const Sexpr {
        self as *const LogicalVector as *const Sexpr
    }
}

impl TryIntoR for bool {
    type Error = <&'static LogicalVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = LogicalVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = if self {
            Rlogical::True
        } else {
            Rlogical::False
        };
        (&vec).try_into_r()
    }
}

impl TryIntoR for Rlogical {
    type Error = <&'static LogicalVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = LogicalVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        (&vec).try_into_r()
    }
}
