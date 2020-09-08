use crate::interface_traits::*;
use crate::internals::c::{R_NaString, Rf_mkCharLenCE};
use crate::internals::sexpr::{SexpType, Sexpr};
use crate::r_types::Rbox;

pub struct RString {
    sexp: *const Sexpr,
}

impl Na for RString {
    fn na() -> Self {
        Self {
            sexp: unsafe { R_NaString },
        }
    }
    fn is_na(&self) -> bool {
        self.sexp == Self::na().sexp
    }
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
        std::str::from_utf8_unchecked((*self.sexp).sxp.vec.get_slice::<u8>())
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

impl TryFromR for &CharacterVector {
    type Error = &'static str;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let ref_expr = unsafe { r.as_mut() }.ok_or("Argument is a null pointer")?;
        if !ref_expr.sxpinfo.alt() {
            match ref_expr.sxpinfo.r#type().unwrap() {
                SexpType::Str => Ok(unsafe { &*(ref_expr as *mut Sexpr as *mut CharacterVector) }),
                _ => Err("Argument is not a character vector"),
            }
        } else {
            Err("Argument is alt rep")
        }
    }
}

impl TryFromR for &[RString] {
    type Error = <&'static CharacterVector as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let nv = <&CharacterVector>::try_from_r(r)?;
        Ok(nv.as_slice())
    }
}

impl TryFromR for &RString {
    type Error = <&'static [RString] as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let slice = <&[RString]>::try_from_r(r)?;
        if slice.len() == 1 {
            Ok(&slice[0])
        } else {
            Err("Was expecting a Character Vector of length 1")
        }
    }
}

impl TryFromR for &str {
    type Error = <&'static RString as TryFromR>::Error;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error> {
        let r_string = <&RString>::try_from_r(r)?;
        Ok(r_string.as_str().unwrap())
    }
}

impl IntoR for &CharacterVector {
    fn into_r(self) -> *const Sexpr {
        self as *const CharacterVector as *const Sexpr
    }
}

impl IntoR for &RString {
    fn into_r(self) -> *const Sexpr {
        self as *const RString as *const Sexpr
    }
}

impl TryIntoR for RString {
    type Error = <&'static CharacterVector as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        let mut vec = CharacterVector::new_rboxed_with_size(1);
        vec.as_mut_slice()[0] = self;
        vec.try_into_r()
    }
}

impl TryIntoR for &str {
    type Error = <RString as TryIntoR>::Error;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error> {
        RString::new(self).try_into_r()
    }
}
