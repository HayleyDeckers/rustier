use crate::interface::*;
use crate::internals::sexpr::{SexpType, Sexpr};
use crate::r_types::Rbox;

pub struct NumericVector {
    sexp: Sexpr,
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

impl FromR for &NumericVector {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let ref_expr = unsafe { r.as_mut() }?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Real => Some(unsafe { &*(ref_expr as *mut Sexpr as *mut NumericVector) }),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl FromR for &[f64] {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let nv = <&NumericVector>::from_r(r)?;
        Some(nv.as_slice())
    }
}

impl FromR for f64 {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let slice = <&[f64]>::from_r(r)?;
        if slice.len() == 1 {
            Some(slice[0])
        } else {
            None
        }
    }
}

impl ReturnableToR for &NumericVector {
    fn return_to_r(self) -> *const Sexpr {
        self as *const NumericVector as *const Sexpr
    }
}

impl ReturnableToR for f64 {
    fn return_to_r(self) -> *const Sexpr {
        let mut vec = NumericVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        vec.return_to_r()
    }
}
