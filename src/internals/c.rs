use crate::internals::sexpr::{SexpType, Sexpr};

// https://github.com/wch/r-source/blob/abb550c99b3927e5fc03d12f1a8e7593fddc04d2/src/main/arithmetic.c#L337
pub const R_NaInt: i32 = std::i32::MIN;
//not guarenteed, but commited 22+ years ago and not changed since.
//https://github.com/wch/r-source/blame/abb550c99b3927e5fc03d12f1a8e7593fddc04d2/src/main/arithmetic.c#L115
pub fn R_NaReal() -> f64 {
    f64::from_bits(0x7ff00000u64 | 0x1954)
}
//https://github.com/wch/r-source/blob/5a156a0865362bb8381dcd69ac335f5174a4f60c/src/include/R_ext/Arith.h#L55
pub const R_NaLogical: i32 = R_NaInt;
#[link(name = "R")]
extern "C" {
    //https://github.com/wch/r-source/blob/1c5c63c365a6bb0eb488533a37677ed4d8f1406d/src/main/names.c#L1199
    pub static mut R_NaString: *const Sexpr;
    pub static mut R_NilValue: *const Sexpr;
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...);
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
