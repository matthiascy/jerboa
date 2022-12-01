use crate::core::{ArrCore, CShape, FixedSized, Layout, RowMajor, Scalar, TLayout};
use core::fmt::{Debug, Formatter};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S, L = RowMajor>(ArrCore<FixedSized<A, { <S as CShape>::N_ELEMS }>, S, L>)
where
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:;

impl<A, S, L> Array<A, S, L>
where
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub const fn new(data: [A; <S as CShape>::N_ELEMS]) -> Self {
        let strides = match L::LAYOUT {
            Layout::RowMajor => <S as CShape>::ROW_MAJOR_STRIDES,
            Layout::ColumnMajor => <S as CShape>::COLUMN_MAJOR_STRIDES,
        };
        Self(ArrCore {
            data: FixedSized(data),
            shape: <S as CShape>::SHAPE,
            strides,
            layout: L::LAYOUT,
            _marker: std::marker::PhantomData,
        })
    }

    /// Creates a new array from a slice.
    pub const fn from_slice(slice: &[A]) -> Self
    where
        A: Clone,
    {
        assert!(slice.len() >= <S as CShape>::N_ELEMS, "slice is too short");
        let mut data = core::mem::MaybeUninit::<[A; <S as CShape>::N_ELEMS]>::uninit();
        let data_ptr = data.as_mut_ptr() as *mut A;
        unsafe {
            data_ptr.copy_from_nonoverlapping(slice.as_ptr(), <S as CShape>::N_ELEMS);
            Self::new(data.assume_init())
        }
    }
}

impl<A, S, L> From<&[A]> for Array<A, S, L>
where
    A: Clone,
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    fn from(slice: &[A]) -> Self {
        Self::from_slice(slice)
    }
}

mod ops {
    use super::*;
    use core::ops::{Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Mul, Rem, Shl, Shr, Sub};

    macro dispatch_array_core_binary_op($tr:ident, $op:tt, $mth:ident) {
        impl<A, B, S, L> $tr<B> for Array<A, S, L>
            where
                A: $tr<B, Output = A> + Clone,
                B: Scalar,
                L: TLayout,
                S: CShape,
                [(); <S as CShape>::N_ELEMS]:,
        {
           type Output = Self;

           fn $mth(self, rhs: B) -> Self::Output {
               Self(self.0 $op rhs)
           }
        }

        impl<'a, A, B, S, L> $tr<B> for &'a Array<A, S, L>
            where
                A: $tr<B, Output = A> + Clone,
                B: Scalar,
                L: TLayout,
                S: CShape,
                [(); <S as CShape>::N_ELEMS]:,
        {
            type Output = Array<A, S, L>;

            fn $mth(self, rhs: B) -> Self::Output {
                Array::<A, S, L>(&self.0 $op rhs)
            }
        }
    }

    // impl<'a, A, B, S, L> Add<B> for &'a Array<A, S, L>
    // where
    //     A: Add<B, Output = A> + Clone,
    //     B: Scalar,
    //     L: TLayout,
    //     S: CShape,
    //     [(); <S as CShape>::N_ELEMS]:,
    // {
    //     type Output = Array<A, S, L>;

    //     fn add(self, rhs: B) -> Self::Output {
    //         Array::<A, S, L>(&self.0 + rhs)
    //     }
    // }

    dispatch_array_core_binary_op!(Add, +, add);
    dispatch_array_core_binary_op!(Sub, -, sub);
    dispatch_array_core_binary_op!(Mul, *, mul);
    dispatch_array_core_binary_op!(Div, /, div);
    dispatch_array_core_binary_op!(Rem, %, rem);
    dispatch_array_core_binary_op!(BitAnd, &, bitand);
    dispatch_array_core_binary_op!(BitOr, |, bitor);
    dispatch_array_core_binary_op!(BitXor, ^, bitxor);
    dispatch_array_core_binary_op!(Shl, <<, shl);
    dispatch_array_core_binary_op!(Shr, >>, shr);

    impl<A, S, L> Deref for Array<A, S, L>
    where
        L: TLayout,
        S: CShape,
        [(); <S as CShape>::N_ELEMS]:,
    {
        type Target = ArrCore<FixedSized<A, { <S as CShape>::N_ELEMS }>, S, L>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<A, S, L> DerefMut for Array<A, S, L>
    where
        L: TLayout,
        S: CShape,
        [(); <S as CShape>::N_ELEMS]:,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

pub use ops::*;

impl<A, S, L> Debug for Array<A, S, L>
where
    A: Debug,
    L: TLayout,
    S: CShape,
    <S as CShape>::UnderlyingType: Debug,
    [(); <S as CShape>::N_ELEMS]:,
    [(); <S as CShape>::N_ELEMS]:,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Array")
            .field("data", &self.data)
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .field("layout", &self.layout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{s, ColumnMajor};

    #[test]
    fn new() {
        let array = Array::<u32, s!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.shape, [2, 5]);
        assert_eq!(array.shape(), &[2, 5]);
    }

    #[test]
    #[should_panic]
    fn from_slice_panic() {
        let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let _array = Array::<u32, s!(3, 4)>::from_slice(&a);
    }

    #[test]
    fn from_slice() {
        let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array = Array::<u32, s!(2, 4)>::from_slice(&a);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn from_trait() {
        let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array: Array<u32, s!(2, 4)> = a[..8].into();
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7]);
        let array2 = Array::<u32, s!(4, 2), ColumnMajor>::from(a.as_slice());
        assert_eq!(array2.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn n_elems() {
        let array: Array<f32, s!(3, 2, 4)> = Array::new([0.0; 24]);
        assert_eq!(array.n_elems(), 24);
    }

    #[test]
    fn add() {
        let a = Array::<u32, s!(2, 4)>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);
        let b = a + 1;
        assert_eq!(b.0.data.0, [1, 2, 3, 4, 5, 6, 7, 8]);


        let c = Array::<u32, s!(2, 4)>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);
        let d = &c + 11;
        assert_eq!(d.0.data.0, [11, 12, 13, 14, 15, 16, 17, 18]);
    }

    #[test]
    fn sub() {
        let a = Array::<i32, s!(2, 4)>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);
        let b = a - 1;
        assert_eq!(b.0.data.0, [-1, 0, 1, 2, 3, 4, 5, 6]);


        let c = Array::<i32, s!(2, 4)>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);
        let d = &c - 20;
        assert_eq!(d.0.data.0, [-20, -19, -18, -17, -16, -15, -14, -13]);
    }

    // todo: proptest
}
