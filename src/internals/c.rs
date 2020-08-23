use crate::internals::sexpr::{SexpType, Sexpr};

#[link(name = "R")]
extern "C" {
    //should be const? maybe..
    pub static mut R_NaReal: f64;
    pub static mut R_NaInt: ::std::os::raw::c_int;
    pub static mut R_NaString: *const Sexpr;
    pub static mut R_NilValue: *const Sexpr;
    //TODO: consider R_PreserveObject instead?
    // takes muts, really. All SEXPR are *mut Sexpr! check which are const safe?
    pub fn Rf_protect(s: *mut Sexpr) -> *mut Sexpr;
    pub fn Rf_unprotect_ptr(arg1: *mut Sexpr);
    pub fn Rf_allocVector(sexp_type: SexpType, size: isize) -> *mut Sexpr; //can _this one_ return null?
    pub fn Rf_mkCharLenCE(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: u32,
    ) -> *mut Sexpr;
}
