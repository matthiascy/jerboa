use crate::core::{arith::Scalar, ArrayCore, DataMut, Shape, TLayout};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

// todo:
//  + scalar left hand side ops
//  + neg
//  + array & array ops with broadcasting

macro impl_binary_op($tr:ident, $op:tt, $mth:ident) {
    impl<A, B, D, S, L> $tr<B> for ArrayCore<D, S, L>
    where
        A: $tr<B, Output = A> + Clone,
        B: Scalar,
        D: DataMut<Elem = A>,
        L: TLayout,
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

    // impl<'a, A, B, D, S> $tr<B> for &'a ArrayCore<D, S>
    // where A: $tr<B, Output = A> + Clone,
    //       B: Scalar,
    //       D: DataMut<Elem = A>,
    //       S: Shape,
    // {
    //     type Output = ArrayCore<D, S>;
    //
    //     fn $mth(self, rhs: B) -> Self::Output {
    //         let mut data = self.data.clone();
    //         for elem in data.as_mut_slice() {
    //             *elem = elem.clone() $op rhs.clone();
    //         }
    //         array
    //     }
    // }
}

impl<'a, A, B, D, S, L> Add<B> for &'a ArrayCore<D, S, L>
    where A: Add<B, Output = A> + Clone,
          B: Scalar,
          D: DataMut<Elem = A>,
          L: TLayout,
          S: Shape,
{
    type Output = ArrayCore<D, S>;

    fn add(self, rhs: B) -> Self::Output {
        let mut data = self.data.clone();
        for elem in data.as_mut_slice() {
            *elem = elem.clone() + rhs.clone();
        }
        array
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
    use crate::{core::s, Array, ArrayD, ArrayDyn};

    #[test]
    fn add() {
        let a = ArrayDyn::from_slice(&[2, 5], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let b = a + 1;
        println!("{:?}", b);

        let a: Array<i32, s!(2, 2)> = Array::new([1, 2, 3, 4]);
        let b = a + 1;
        assert_eq!(b.data, Array::<i32, s!(2, 2)>::new([2, 3, 4, 5]).data);

        let a: ArrayD<f32, s!(2, 3)> = ArrayD::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = a + 2.0;
        assert_eq!(
            b.data,
            ArrayD::<f32, s!(2, 3)>::new([3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).data
        );
    }
}
