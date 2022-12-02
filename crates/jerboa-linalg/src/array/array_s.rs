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
            marker: std::marker::PhantomData,
        })
    }

    /// Creates a new array by cloning the elements of a slice.
    pub const fn from_slice_clone(slice: &[A]) -> Self
    where
        A: ~const Clone,
    {
        assert!(slice.len() >= <S as CShape>::N_ELEMS, "input slice is too short");
        let mut data = core::mem::MaybeUninit::<[A; <S as CShape>::N_ELEMS]>::uninit();
        let data_ptr = data.as_mut_ptr() as *mut A;
        unsafe {
            let mut i = 0;
            while i < <S as CShape>::N_ELEMS {
                data_ptr.add(i).write(slice[i].clone());
                i += 1;
            }
            Self::new(data.assume_init())
        }
    }

    /// Creates a new array by copying the elements of a slice.
    pub const fn from_slice_copy(slice: &[A]) -> Self
    where
        A: Clone + Copy,
    {
        assert!(slice.len() >= <S as CShape>::N_ELEMS, "input slice is too short");
        let mut data = core::mem::MaybeUninit::<[A; <S as CShape>::N_ELEMS]>::uninit();
        unsafe {
            (data.as_mut_ptr() as *mut A).copy_from_nonoverlapping(slice.as_ptr(), <S as CShape>::N_ELEMS);
            Self::new(data.assume_init())
        }
    }

    /// Extracts a slice containing the entire array data.
    pub const fn as_slice(&self) -> &[A] {
        &self.0.data.0
    }
}

impl<A, S, L> const Clone for Array<A, S, L>
where
    A: ~const Clone,
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    fn clone(&self) -> Self {
        Self::from_slice_clone(self.as_slice())
    }
}

impl<A, S, L> Copy for Array<A, S, L>
where
    A: Copy,
    L: TLayout,
    S: CShape,
    S::UnderlyingType: Copy,
    [(); <S as CShape>::N_ELEMS]:,
{ }

impl<A, S, L> From<&[A]> for Array<A, S, L>
where
    A: Clone,
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    default fn from(slice: &[A]) -> Self {
        Self::from_slice_clone(slice)
    }
}

impl<A, S, L> From<&[A]> for Array<A, S, L>
where
    A: Copy,
    L: TLayout,
    S: CShape,
    S::UnderlyingType: Copy,
    [(); <S as CShape>::N_ELEMS]:,
{
    fn from(slice: &[A]) -> Self {
        Self::from_slice_copy(slice)
    }
}

impl<A, S0, S1, L0, L1> PartialEq<Array<A, S1, L1>> for Array<A, S0, L0>
where
    A: PartialEq,
    L0: TLayout,
    L1: TLayout,
    S0: CShape,
    S1: CShape,
    [(); <S0 as CShape>::N_ELEMS]:,
    [(); <S1 as CShape>::N_ELEMS]:,
{
    fn eq(&self, other: &Array<A, S1, L1>) -> bool {
        self.0.eq(&other.0)
    }
}

mod ops {
    use super::*;
    use core::ops::{Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Mul, Rem, Shl, Shr, Sub};

    macro impl_array_binary_op($tr:ident, $op:tt, $mth:ident) {
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
                Array(&self.0 $op rhs)
            }
        }
    }

    impl_array_binary_op!(Add, +, add);
    impl_array_binary_op!(Sub, -, sub);
    impl_array_binary_op!(Mul, *, mul);
    impl_array_binary_op!(Div, /, div);
    impl_array_binary_op!(Rem, %, rem);
    impl_array_binary_op!(BitAnd, &, bitand);
    impl_array_binary_op!(BitOr, |, bitor);
    impl_array_binary_op!(BitXor, ^, bitxor);
    impl_array_binary_op!(Shl, <<, shl);
    impl_array_binary_op!(Shr, >>, shr);

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
    use proptest::prelude::*;

    const HALF_U32: u32 = u32::MAX / 2 - 1;

    fn array2x3x4u32(min: u32, max: u32) -> impl Strategy<Value = Array<u32, s!(2, 3, 4)>> {
        prop::array::uniform32(min..max).prop_map(|a| Array::from_slice_clone(&a))
    }

    fn array4x6_lower() -> impl Strategy<Value = Array<u32, s!(4, 6)>> {
        prop::array::uniform32(0..HALF_U32).prop_map(|a| Array::from_slice_clone(&a))
    }

    fn array4x6_upper() -> impl Strategy<Value = Array<u32, s!(4, 6)>> {
        prop::array::uniform32(HALF_U32..u32::MAX).prop_map(|a| Array::from_slice_clone(&a))
    }

    proptest! {
        #[test]
        fn new(a in any::<[f32; 10]>()) {
            let array = Array::<f32, s!(2, 5)>::new(a);
            assert_eq!(array.data.0, a);
            assert_eq!(array.shape, [2, 5]);
            assert_eq!(array.shape(), &[2, 5]);
        }

        #[test]
        fn partial_eq(a in any::<[i32; 28]>()) {
            let array1 = Array::<i32, s!(2, 2, 7)>::new(a);
            let array2 = Array::<i32, s!(2, 2, 7)>::new(a);
            let array3 = Array::<i32, s!(7, 2, 2), ColumnMajor>::new(a);
            assert_eq!(array1, array2);
            assert_eq!(array1, array3);
            assert_eq!(array2, array3);
        }

        #[test]
        #[should_panic]
        fn from_slice_panic(a in any::<[i64; 16]>()) {
            let _array = Array::<i64, s!(3, 4, 2, 2)>::from_slice_clone(&a);
        }

        #[test]
        fn from_slice(a in any::<[f64; 20]>()) {
            let array0 = Array::<f64, s!(4, 4)>::from_slice_clone(&a[..]);
            let array1 = Array::<f64, s!(4, 3)>::from_slice_clone(&a[..]);
            let array2 = Array::<f64, s!(4, 3, 1)>::from_slice_clone(&a[..]);
            let array3 = Array::<f64, s!(2, 2, 2, 2)>::from_slice_clone(&a[..]);
            assert_eq!(array0.0.data.0, a[0..16]);
            assert_eq!(array1.0.data.0, a[0..12]);
            assert_eq!(array2.0.data.0, a[0..12]);
            assert_eq!(array3.0.data.0, a[0..16]);
        }

        #[test]
        fn from_trait(a in any::<[f32; 16]>()) {
            let array0: Array<f32, s!(2, 4)> = a[..8].into();
            assert_eq!(array0.0.data.0, a[..8]);

            let array1 = Array::<f32, s!(4, 2), ColumnMajor>::from(a.as_slice());
            assert_eq!(array1.0.data.0, a[..8]);
        }

        #[test]
        fn n_elems(a in any::<[f32; 32]>()) {
            let array: Array<f32, s!(3, 2, 4)> = Array::from(&a[..]);
            assert_eq!(array.n_elems(), 24);

            let array: Array<f32, s!(2, 4, 4), ColumnMajor> = Array::from(&a[..]);
            assert_eq!(array.n_elems(), 32);

            let array: Array<f32, s!(2, 2, 4), ColumnMajor> = Array::from(&a[..]);
            assert_eq!(array.n_elems(), 16);
        }

        #[test]
        fn add_scalar_0(arr in array2x3x4u32(0, HALF_U32), b in 0..HALF_U32) {
            let c = &arr + b;
            let d = arr + b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn add_scalar_1(arr in array4x6_lower(), b in 0..HALF_U32) {
            let c = &arr + b;
            let d = arr + b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn sub_scalar_0(arr in array2x3x4u32(HALF_U32, u32::MAX), b in 0..HALF_U32) {
            let c = &arr - b;
            let d = arr - b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn sub_scalar_1(arr in array4x6_upper(), b in 0..HALF_U32) {
            let c = &arr - b;
            let d = arr - b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn mul_scalar_0(arr in array2x3x4u32(0, HALF_U32 / 2), b in 0..4u32) {
            let c = &arr * b;
            let d = arr * b;
            prop_assert_eq!(c, d);
        }
    }

    // Test that the array is using the correct constructor when
    // the data is copyable or cloneable.
    #[test]
    fn copy_clone_slice() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let _arr_0 = Array::<i32, s!(2, 5)>::from(&a[..]);
        let b = [Box::new(1), Box::new(2), Box::new(3), Box::new(4), Box::new(5), Box::new(6)];
        let _arr_1 = Array::<Box<u32>, s!(2, 3)>::from(&b[..]);
    }
}
