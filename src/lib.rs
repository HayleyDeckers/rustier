pub mod interface;
mod internals;
mod r_types;
pub use internals::sexpr;
pub use r_types::*;

// pub trait IsRNa {
//     fn is_na(&self) -> bool;
// }

// impl IsRNa for f64 {
//     fn is_na(&self) -> bool {
//         unsafe { *self == R_NaReal }
//     }
// }
// impl IsRNa for i32 {
//     fn is_na(&self) -> bool {
//         unsafe { *self == R_NaInt }
//     }
// }
// impl IsRNa for sexpr::Sexpr {
//     fn is_na(&self) -> bool {
//         unsafe { (self as *const sexpr::Sexpr) == R_NaString }
//     }
// }

pub use crate::interface::*;
pub use crate::r_types::numerics::NumericVector;
pub use crate::r_types::strings::*;
use r_macros::R_export;

#[R_export]
fn hello_world() -> &'static str {
    "Hello, World!"
}

#[R_export]
fn fancy_hello_world() -> Rbox<CharacterVector> {
    let mut cv = CharacterVector::new_rboxed_with_size(3);
    let slice = cv.as_mut_slice();
    slice[0] = RString::new("من left اليمين to الى right اليسار");
    slice[1] = RString::new("🏳️‍⚧️");
    slice[2] = RString::new("🦀");
    cv
}

#[R_export]
fn sum_slice(slice: &[f64]) -> f64 {
    slice.into_iter().sum()
}

#[R_export]
fn max_nv(nv: &NumericVector) -> f64 {
    nv.as_slice()
        .into_iter()
        .fold(f64::NEG_INFINITY, |max: f64, x: &f64| {
            if !x.is_nan() && (*x > max) {
                *x
            } else {
                max
            }
        })
}
#[R_export]
fn ops(value: f64, second: f64) -> Rbox<NumericVector> {
    let mut nv = Rbox::<NumericVector>::new_with_size(4);
    let slice = nv.as_mut_slice();
    slice[0] = value + second;
    slice[1] = value - second;
    slice[2] = value * second;
    slice[3] = value / second;
    nv
}

//see aout adding SIMD https://stackoverflow.com/questions/51253203/is-it-possible-to-combine-rayon-and-faster
use rayon::prelude::*;
#[R_export]
fn rayon(v: &[f64], min: f64) -> f64 {
    let num_over_min = v.par_iter().filter(|&&x| x > min).count() as f64;
    let sum_over_min: f64 = v.par_iter().filter(|&&x| x > min).sum();
    sum_over_min / num_over_min
}
