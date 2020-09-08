
// types

mod r_types;

pub mod types {
    pub use crate::r_types::integers::*;
    pub use crate::r_types::logicals::*;
    pub use crate::r_types::numerics::NumericVector;
    pub use crate::r_types::strings::*;
    pub use crate::r_types::*;
}

// macro & friends

mod interface_traits;
mod internals;

pub mod macros {
    pub use rustier_macros::R_export;
    pub mod prelude {
        pub use crate::interface_traits::*;
        pub use crate::internals::c::{R_NilValue, Rf_error};
        pub use crate::internals::sexpr;
    }
}
