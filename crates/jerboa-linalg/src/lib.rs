#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(array_zip)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_intoiterator_identity)]
#![feature(mem_copy_fn)]
#![feature(decl_macro)]

// todo: rewrite some of the code to use const_trait_impl,
// const_fn_floating_point_arithmetic and const_float_classify

// - [TODO] const_trait_impl once it's stable

mod array;
mod num;
mod packet;

pub use num::MachineEpsilon;
