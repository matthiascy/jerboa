use crate::core::{ArrCore, DataMut, DataRawMut, DataClone, TLayout, Scalar, Shape};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

// todo:
//  + scalar left hand side ops
//  + neg
//  + array & array ops with broadcasting

/// Macro for implementing binary ops for arrays.
///
/// Implementations try to avoid unnecessary allocations by
/// reusing the consumed array if possible.
macro impl_binary_op($tr:ident, $op:tt, $mth:ident) {
    impl<A, B, D, S, L> $tr<B> for ArrCore<D, S, L>
    where
        A: $tr<B, Output = A> + Clone,
        B: Scalar,
        D: DataRawMut<Elem = A>,
        L: TLayout,
        S: Shape,
    {
        type Output = Self;

        fn $mth(self, rhs: B) -> Self::Output {
            let mut array = self;
            let n_elems = array.n_elems();
            for i in 0..n_elems {
                unsafe {
                    let elem = core::ptr::read(array.data.as_ptr().add(i));
                    core::ptr::write(array.data.as_mut_ptr().add(i), elem $op rhs.clone());
                }
            }
            array
        }
    }

    impl<'a, A, B, D, S, L> $tr<B> for &'a ArrCore<D, S, L>
    where A: $tr<B, Output = A> + Clone,
          B: Scalar,
          D: DataClone<Elem = A>,
          L: TLayout,
          S: Shape,
    {
        type Output = ArrCore<D, S, L>;

        fn $mth(self, rhs: B) -> Self::Output {
            let mut data = self.data.clone();
            for elem in data.as_mut_slice() {
                *elem = elem.clone() $op rhs.clone();
            }
            ArrCore {
                data,
                shape: self.shape.clone(),
                strides: self.strides.clone(),
                layout: self.layout.clone(),
                _marker: core::marker::PhantomData,
            }
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
