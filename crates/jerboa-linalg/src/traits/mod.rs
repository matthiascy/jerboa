mod arith;
mod num;

pub use arith::*;

pub use num::*;

// pub trait NdArray {
//     fn zeros() -> Self;
// }

// pub trait ArrayDim {
//     const DIM: usize;
// }
//
// impl<T> ArrayDim for T {
//     default const DIM: usize = 0;
// }

// /// Get the underlying value type of the reference.
// pub trait Decay {
//     type Type;
// }
//
// impl<T> Decay for T {
//     default type Type = T;
// }
//
// impl<'a, T> Decay for &'a T {
//     type Type = <T as Decay>::Type;
// }
//
// impl<'a, T> Decay for &'a mut T {
//     type Type = <T as Decay>::Type;
// }
//
// pub trait ArrayScalar {
//     type Scalar;
// }
//
// impl<T> ArrayScalar for T {
//     default type Scalar = <T as Decay>::Type;
// }
//
// pub trait LenHint<const MIN: usize> {}
//
// pub trait ConstCheck<const COND: bool> {}
//
// pub struct IsSameType<A, B>(PhantomData<(A, B)>);
//
// impl<T> ConstCheck<true> for IsSameType<T, T> {}
