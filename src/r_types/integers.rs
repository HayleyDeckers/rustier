use crate::interface_traits::*;
use crate::internals::{
    c::R_NaInt,
    sexpr::{SexpType, Sexpr},
};
use crate::r_types::Rbox;

pub struct IntegerVector {
    sexp: Sexpr,
}
#[derive(Clone, Copy)]
pub struct RInt {
    raw: i32,
}

impl Na for RInt {
    fn na() -> RInt {
        Self { raw: R_NaInt }
    }
    fn is_na(&self) -> bool {
        self.raw == Self::na().raw
    }
}

impl RInt {
    pub unsafe fn as_i32_unchecked(self) -> i32 {
        self.raw
    }
    pub fn as_i32(self) -> Option<i32> {
        if self.is_na() {
            None
        } else {
            Some(self.raw)
        }
    }
}

impl IntegerVector {
    pub fn new_rboxed_with_size(size: isize) -> Rbox<Self> {
        Rbox::<Self>::new_with_size(size)
    }
    pub fn as_slice(&self) -> &[RInt] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_mut_slice(&mut self) -> &mut [RInt] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
    pub fn as_raw_slice(&self) -> &[i32] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_raw_mut_slice(&mut self) -> &mut [i32] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
}

impl TryFromR for &IntegerVector {
    type Error = &'static str;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let ref_expr = unsafe { r.as_mut() }.ok_or("Argument is a null pointer")?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Int => Ok(unsafe { &*(ref_expr as *mut Sexpr as *mut IntegerVector) }),
                _ => Err("Argument is not a Integer vector"),
            }
        } else {
            Err("Argument is alt rep")
        }
    }
}

impl TryFromR for &[i32] {
    type Error = <&'static IntegerVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let nv = <&IntegerVector>::try_from_r(r)?;
        Ok(nv.as_raw_slice())
    }
}

impl TryFromR for &[RInt] {
    type Error = <&'static IntegerVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let nv = <&IntegerVector>::try_from_r(r)?;
        Ok(nv.as_slice())
    }
}

impl TryFromR for i32 {
    type Error = <&'static IntegerVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let slice = <&[i32]>::try_from_r(r)?;
        match slice.len() {
            1 => Ok(slice[0]),
            _x => Err("Was expecting a Integer vector of size 1"),
        }
    }
}
impl TryFromR for RInt {
    type Error = <&'static IntegerVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        Ok(RInt {
            raw: <i32 as TryFromR>::try_from_r(r)?,
        })
    }
}

impl IntoR for &IntegerVector {
    fn into_r(self) -> *const Sexpr {
        self as *const IntegerVector as *const Sexpr
    }
}

impl TryIntoR for i32 {
    type Error = <&'static IntegerVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = IntegerVector::new_rboxed_with_size(1);
        vec.as_raw_mut_slice()[0] = self;
        (&vec).try_into_r()
    }
}

impl TryIntoR for RInt {
    type Error = <&'static IntegerVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = IntegerVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        (&vec).try_into_r()
    }
}
