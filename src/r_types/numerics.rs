use crate::interface_traits::*;
use crate::internals::{
    c::R_NaReal,
    sexpr::{SexpType, Sexpr},
};
use crate::r_types::Rbox;

pub struct NumericVector {
    sexp: Sexpr,
}

impl Na for f64 {
    fn na() -> Self {
        R_NaReal()
    }
    fn is_na(&self) -> bool {
        //todo: fix this at certain value?
        self.to_bits() == Self::na().to_bits()
    }
}

impl NumericVector {
    pub fn new_rboxed_with_size(size: isize) -> Rbox<Self> {
        Rbox::<Self>::new_with_size(size)
    }
    pub fn as_slice(&self) -> &[f64] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
}

impl TryFromR for &NumericVector {
    type Error = &'static str;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let ref_expr = unsafe { r.as_mut() }.ok_or("Argument is a null pointer")?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Real => Ok(unsafe { &*(ref_expr as *mut Sexpr as *mut NumericVector) }),
                _ => Err("Argument is not a Numeric vector"),
            }
        } else {
            Err("Argument is alt rep")
        }
    }
}

impl TryFromR for &[f64] {
    type Error = <&'static NumericVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let nv = <&NumericVector>::try_from_r(r)?;
        Ok(nv.as_slice())
    }
}

impl TryFromR for f64 {
    type Error = <&'static NumericVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let slice = <&[f64]>::try_from_r(r)?;
        match slice.len() {
            1 => Ok(slice[0]),
            _x => Err("Was expecting a numeric vector of size 1"),
        }
    }
}

impl IntoR for &NumericVector {
    fn into_r(self) -> *const Sexpr {
        self as *const NumericVector as *const Sexpr
    }
}

impl TryIntoR for f64 {
    type Error = <&'static NumericVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = NumericVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        (&vec).try_into_r()
    }
}
