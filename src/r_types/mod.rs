use crate::internals::{
    c::*,
    sexpr::{SexpType, Sexpr},
};
pub mod integers;
pub mod logicals;
pub mod numerics;
pub mod strings;

pub struct Rbox<T> {
    ptr: *mut Sexpr,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> std::ops::Deref for Rbox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr as *mut T as *const T) }
    }
}

impl<T> std::ops::DerefMut for Rbox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

impl Rbox<numerics::NumericVector> {
    pub fn new_with_size(size: isize) -> Self {
        //todo: checks
        Rbox {
            ptr: unsafe { Rf_protect(Rf_allocVector(SexpType::Real, size)) },
            _phantom: std::marker::PhantomData,
        }
    }
}
impl Rbox<integers::IntegerVector> {
    pub fn new_with_size(size: isize) -> Self {
        //todo: checks
        Rbox {
            ptr: unsafe { Rf_protect(Rf_allocVector(SexpType::Int, size)) },
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Rbox<logicals::LogicalVector> {
    pub fn new_with_size(size: isize) -> Self {
        //todo: checks
        Rbox {
            ptr: unsafe { Rf_protect(Rf_allocVector(SexpType::Lgl, size)) },
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Rbox<strings::CharacterVector> {
    pub fn new_with_size(size: isize) -> Self {
        //todo: checks
        Rbox {
            ptr: unsafe { Rf_protect(Rf_allocVector(SexpType::Str, size)) },
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> std::ops::Drop for Rbox<T> {
    fn drop(&mut self) {
        unsafe { Rf_unprotect_ptr(self.ptr) }
    }
}
