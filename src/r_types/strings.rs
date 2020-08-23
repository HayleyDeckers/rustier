use crate::interface::*;
use crate::internals::c::Rf_mkCharLenCE;
use crate::internals::sexpr::{SexpType, Sexpr};
use crate::r_types::Rbox;

pub struct RString {
    sexp: *const Sexpr,
}

impl RString {
    pub fn new<S: core::ops::Deref<Target = str>>(content: S) -> Self {
        RString {
            sexp: unsafe {
                Rf_mkCharLenCE(content.as_ptr() as *mut i8, content.len() as i32, 1) as *const Sexpr
            },
        }
    }
    pub unsafe fn as_str_unchecked(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked((*self.sexp).sxp.vec.get_slice::<u8>()) }
    }
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { std::str::from_utf8((*self.sexp).sxp.vec.get_slice::<u8>()) }
    }
}

pub struct CharacterVector {
    sexp: Sexpr,
}

impl CharacterVector {
    pub fn new_rboxed_with_size(size: isize) -> Rbox<CharacterVector> {
        Rbox::<CharacterVector>::new_with_size(size)
    }
    pub fn as_slice(&self) -> &[RString] {
        unsafe { self.sexp.sxp.vec.get_slice() }
    }
    pub fn as_mut_slice(&mut self) -> &mut [RString] {
        unsafe { self.sexp.sxp.vec.get_mut_slice() }
    }
}

impl FromR for &CharacterVector {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let ref_expr = unsafe { r.as_mut() }?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Str => {
                    Some(unsafe { &*(ref_expr as *mut Sexpr as *mut CharacterVector) })
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl FromR for &[RString] {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let nv = <&CharacterVector>::from_r(r)?;
        Some(nv.as_slice())
    }
}

impl FromR for &RString {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let slice = <&[RString]>::from_r(r)?;
        if slice.len() == 1 {
            Some(&slice[0])
        } else {
            None
        }
    }
}

impl FromR for &str {
    fn from_r(r: *mut Sexpr) -> Option<Self> {
        let r_string = <&RString>::from_r(r)?;
        Some(r_string.as_str().unwrap())
    }
}

impl ReturnableToR for &CharacterVector {
    fn return_to_r(self) -> *const Sexpr {
        self as *const CharacterVector as *const Sexpr
    }
}

impl ReturnableToR for &str {
    fn return_to_r(self) -> *const Sexpr {
        RString::new(self).return_to_r()
    }
}

impl ReturnableToR for &RString {
    fn return_to_r(self) -> *const Sexpr {
        self as *const RString as *const Sexpr
    }
}

impl ReturnableToR for RString {
    fn return_to_r(self) -> *const Sexpr {
        let mut vec = CharacterVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        vec.return_to_r()
    }
}
