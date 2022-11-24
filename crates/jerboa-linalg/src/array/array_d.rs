use crate::core::{ArrayCore, CShape, DynSized, Layout, RowMajor, TLayout};
use std::fmt::{Debug, Formatter};

/// Fix-sized array on the heap.
#[repr(transparent)]
pub struct ArrayD<A, S: CShape, L: TLayout = RowMajor>(ArrayCore<DynSized<A>, S, L>);

impl<A, S, L> ArrayD<A, S, L>
where
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as CShape>::N_ELEMS]) -> Self {
        let strides = match L::LAYOUT {
            Layout::RowMajor => <S as CShape>::ROW_MAJOR_STRIDES,
            Layout::ColumnMajor => <S as CShape>::COLUMN_MAJOR_STRIDES,
        };
        Self(ArrayCore {
            data: DynSized(Vec::from(data)),
            shape: <S as CShape>::SHAPE,
            strides,
            layout: L::LAYOUT,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<A, S, L> Debug for ArrayD<A, S, L>
where
    L: TLayout,
    S: CShape,
    <S as CShape>::UnderlyingType: Debug,
    [(); <S as CShape>::N_ELEMS]:,
    A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayD")
            .field("data", &self.data)
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .field("layout", &self.layout)
            .finish()
    }
}

mod ops {
    use super::ArrayD;
    use crate::core::{ArrayCore, CShape, DynSized, Scalar, TLayout};
    use core::ops::{Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Mul, Rem, Shl, Shr, Sub};

    impl<A, S, L> Deref for ArrayD<A, S, L>
    where
        L: TLayout,
        S: CShape,
        [(); <S as CShape>::N_ELEMS]:,
    {
        type Target = ArrayCore<DynSized<A>, S, L>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<A, S, L> DerefMut for ArrayD<A, S, L>
    where
        L: TLayout,
        S: CShape,
        [(); <S as CShape>::N_ELEMS]:,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    macro impl_array_d_binary_op($tr:ident, $mth:ident) {
        impl<A, B, S, L> $tr<B> for ArrayD<A, S, L>
        where
            A: $tr<B, Output = A> + Clone,
            B: Scalar,
            L: TLayout,
            S: CShape,
            [(); <S as CShape>::N_ELEMS]:,
        {
            type Output = Self;

            fn $mth(self, rhs: B) -> Self::Output {
                Self(self.0.$mth(rhs))
            }
        }

        // impl<'a, A, B, S> $tr<B> for &'a ArrayD<A, S>
        // where
        //     A: $tr<B, Output = A> + Clone,
        //     B: Scalar,
        //     S: FixedShape,
        //     [(); <S as FixedShape>::N_ELEMS]:,
        // {
        //     type Output = ArrayD<A, S>;
        //
        //     fn $mth(self, rhs: B) -> Self::Output {
        //         ArrayD(self.0.$mth(rhs))
        //     }
        // }
    }

    impl_array_d_binary_op!(Add, add);
    impl_array_d_binary_op!(Sub, sub);
    impl_array_d_binary_op!(Mul, mul);
    impl_array_d_binary_op!(Div, div);
    impl_array_d_binary_op!(Rem, rem);
    impl_array_d_binary_op!(BitAnd, bitand);
    impl_array_d_binary_op!(BitOr, bitor);
    impl_array_d_binary_op!(BitXor, bitxor);
    impl_array_d_binary_op!(Shl, shl);
    impl_array_d_binary_op!(Shr, shr);
}

pub use ops::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::s;

    #[test]
    fn n_elems() {
        let array: ArrayD<f32, s!(3, 2, 4)> = ArrayD::new([0.0; 24]);
        assert_eq!(array.n_elems(), 24);
    }

    #[test]
    fn add() {
        let array: ArrayD<f32, s!(3, 2, 3)> = ArrayD::new([0.0; 18]);
        let array = array + 1.0;
        assert_eq!(array.data.as_slice(), &[1.0; 18]);
    }
}
