use std::ops::{Add, Sub};
use crate::core::{ArrayCore, Data, DataMut, DataRaw, Shape};

// todo:
//  + scalar left hand side ops
//  + neg
//  + array & array ops with broadcasting

pub trait Scalar: Clone {}

impl Scalar for bool {}
impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}
impl Scalar for usize {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}
impl Scalar for f32 {}
impl Scalar for f64 {}

macro impl_binary_op($tr:ident, $op:tt, $mth:ident) {
    impl<A, B, D, S> $tr<B> for ArrayCore<D, S>
    where
        A: $tr<B, Output = A> + Clone,
        B: Scalar,
        D: DataMut<Elem = A>,
        S: Shape,
    {
        type Output = Self;

        fn $mth(self, rhs: B) -> Self::Output {
            let mut array = self;
            for elem in array.data.as_mut_slice() {
                *elem = elem.clone() $op rhs.clone();
            }
            array
        }
    }
}

impl_binary_op!(Add, +, add);
impl_binary_op!(Sub, -, sub);
impl_binary_op!(Mul, *, mul);
impl_binary_op!(Div, /, div);
impl_binary_op!(Rem, %, rem);
impl_binary_op!(BitAnd, &, bitand);
impl_binary_op!(BitOr, |, bitor);
impl_binary_op!(BitXor, ^, bitxor);
impl_binary_op!(Shl, <<, shl);
impl_binary_op!(Shr, >>, shr);


#[cfg(test)]
mod tests {
    use crate::{Array, ArrayD, ArrayDyn};
    use crate::core::cs;
    use super::*;

    #[test]
    fn add_elem() {
        let a = ArrayDyn::from_slice(&[2, 5], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let b = a + 1;
        println!("{:?}", b);

        let a: Array<i32, cs!(2, 2)> = Array::new([1, 2, 3, 4]);
        let b = a + 1;
        println!("{:?}", b);

        let a: ArrayD<f32, cs!(2, 3)> = ArrayD::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = a - 2.0;
        println!("{:?}", b);
    }
}