use crate::internals::sexpr::Sexpr;
pub trait FromR
where
    Self: Sized,
{
    fn from_r(r: *mut Sexpr) -> Option<Self>;
}

pub trait ReturnableToR {
    fn return_to_r(self) -> *const Sexpr;
}
