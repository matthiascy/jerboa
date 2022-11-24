#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(array_zip)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_intoiterator_identity)]
#![feature(mem_copy_fn)]
#![feature(decl_macro)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]

mod array;
mod axis;
mod core;
pub mod num;

pub use array::*;
pub use crate::core::{Layout, s};
pub use axis::Axis3;
