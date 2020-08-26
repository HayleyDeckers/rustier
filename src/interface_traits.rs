use crate::internals::sexpr::Sexpr;
pub trait TryFromR
where
    Self: Sized,
{
    type Error: std::fmt::Display;
    fn try_from_r(r: *mut Sexpr) -> Result<Self, Self::Error>;
}

pub trait TryIntoR {
    type Error: std::fmt::Display;
    fn try_into_r(self) -> Result<*const Sexpr, Self::Error>;
}

pub trait IntoR {
    fn into_r(self) -> *const Sexpr;
}

pub struct Infalliable {}
impl std::fmt::Display for Infalliable {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}
impl<T: IntoR> TryIntoR for T {
    type Error = Infalliable;
    fn try_into_r(self) -> Result<*const Sexpr, Infalliable> {
        Ok(self.into_r())
    }
}

pub trait Na {
    fn na() -> Self;
    fn is_na(&self) -> bool;
}
