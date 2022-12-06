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
        assert!(
            slice.len() >= <S as CShape>::N_ELEMS,
            "input slice is too short"
        );
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
        assert!(
            slice.len() >= <S as CShape>::N_ELEMS,
            "input slice is too short"
        );
        let mut data = core::mem::MaybeUninit::<[A; <S as CShape>::N_ELEMS]>::uninit();
        unsafe {
            (data.as_mut_ptr() as *mut A)
                .copy_from_nonoverlapping(slice.as_ptr(), <S as CShape>::N_ELEMS);
            Self::new(data.assume_init())
        }
    }

    /// Extracts a slice containing the entire array data.
    pub const fn as_slice(&self) -> &[A] {
        &self.0.data.0
    }
}

// Implementation of `Clone` trait when array elements are `Clone`.
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

// Implementation of `Copy` trait when array elements are `Copy`.
impl<A, S, L> Copy for Array<A, S, L>
where
    A: Copy,
    L: TLayout,
    S: CShape,
    S::UnderlyingType: Copy,
    [(); <S as CShape>::N_ELEMS]:,
{
}

// Conversion from a slice to an array when the element type is not Copy.
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

// Conversion from a slice to an array when the element type is Copy.
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

// Implementation of PartialEq for arrays.
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

    macro impl_array_s_binary_op($tr:ident, $op:tt, $mth:ident) {
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

    impl_array_s_binary_op!(Add, +, add);
    impl_array_s_binary_op!(Sub, -, sub);
    impl_array_s_binary_op!(Mul, *, mul);
    impl_array_s_binary_op!(Div, /, div);
    impl_array_s_binary_op!(Rem, %, rem);
    impl_array_s_binary_op!(BitAnd, &, bitand);
    impl_array_s_binary_op!(BitOr, |, bitor);
    impl_array_s_binary_op!(BitXor, ^, bitxor);
    impl_array_s_binary_op!(Shl, <<, shl);
    impl_array_s_binary_op!(Shr, >>, shr);

    macro impl_scalar_lhs_array_s_binary_op {
        (commutative $t:ty, $tr:ident, $op:tt, $mth:ident) => {
            impl<A, S, L> $tr<Array<A, S, L>> for $t
            where
                A: $tr<$t, Output = A>,
                L: TLayout,
                S: CShape,
            [(); <S as CShape>::N_ELEMS]:,
            {
                type Output = Array<A, S, L>;

                fn $mth(self, rhs: Array<A, S, L>) -> Self::Output {
                    Array(self $op rhs.0)
                }
            }

            impl<'a, A, S, L> $tr<&'a Array<A, S, L>> for $t
            where
                A: Clone,
                for<'e> &'e A: $tr<$t, Output = A>,
                L: TLayout,
                S: CShape,
            [(); <S as CShape>::N_ELEMS]:,
            {
                type Output = Array<A, S, L>;

                fn $mth(self, rhs: &'a Array<A, S, L>) -> Self::Output {
                    Array(self $op &rhs.0)
                }
            }
        },
        (non-commutative $t:ty, $tr:ident, $op:tt, $mth:ident) => {
            impl<S, L> $tr<Array<$t, S, L>> for $t
            where
                L: TLayout,
                S: CShape,
                [(); <S as CShape>::N_ELEMS]:,
            {
                type Output = Array<$t, S, L>;

                fn $mth(self, rhs: Array<$t, S, L>) -> Self::Output {
                    Array(self $op rhs.0)
                }
            }

            impl<'a, S, L> $tr<&'a Array<$t, S, L>> for $t
            where
                L: TLayout,
                S: CShape,
            [(); <S as CShape>::N_ELEMS]:,
            {
                type Output = Array<$t, S, L>;

                fn $mth(self, rhs: &'a Array<$t, S, L>) -> Self::Output {
                    Array(self $op &rhs.0)
                }
            }
        }
    }

    macro impl_integer_lhs_array_s_binary_op($($t:ty),*) {
        $(
            impl_scalar_lhs_array_s_binary_op!(commutative $t, Add, +, add);
            impl_scalar_lhs_array_s_binary_op!(commutative $t, Mul, *, mul);
            impl_scalar_lhs_array_s_binary_op!(commutative $t, BitAnd, &, bitand);
            impl_scalar_lhs_array_s_binary_op!(commutative $t, BitOr, |, bitor);
            impl_scalar_lhs_array_s_binary_op!(commutative $t, BitXor, ^, bitxor);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Sub, -, sub);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Div, /, div);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Rem, %, rem);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Shl, <<, shl);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Shr, >>, shr);
        )*
    }

    macro impl_float_lhs_array_s_binary_op($($t:ty),*) {
        $(
            impl_scalar_lhs_array_s_binary_op!(commutative $t, Add, +, add);
            impl_scalar_lhs_array_s_binary_op!(commutative $t, Mul, *, mul);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Sub, -, sub);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Div, /, div);
            impl_scalar_lhs_array_s_binary_op!(non-commutative $t, Rem, %, rem);
        )*
    }

    impl_float_lhs_array_s_binary_op!(f32, f64);
    impl_scalar_lhs_array_s_binary_op!(commutative bool, BitAnd, &, bitand);
    impl_scalar_lhs_array_s_binary_op!(commutative bool, BitOr, |, bitor);
    impl_scalar_lhs_array_s_binary_op!(commutative bool, BitXor, ^, bitxor);
    impl_integer_lhs_array_s_binary_op!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize);

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
    use core::ops::Range;
    use proptest::prelude::*;

    const HALF_U32: u32 = u32::MAX / 2 - 1;

    fn array32<A>(min: A, max: A) -> impl Strategy<Value = [A; 32]>
    where
        Range<A>: Strategy<Value = A>,
        A: Clone + core::fmt::Debug + 'static,
    {
        prop::array::uniform32(min..max)
    }

    fn array16<A>(min: A, max: A) -> impl Strategy<Value = [A; 16]>
    where
        Range<A>: Strategy<Value = A>,
        A: Clone + core::fmt::Debug + 'static,
    {
        prop::array::uniform16(min..max)
    }

    fn array24<A>(min: A, max: A) -> impl Strategy<Value = [A; 24]>
    where
        Range<A>: Strategy<Value = A>,
        A: Clone + core::fmt::Debug + 'static,
    {
        prop::array::uniform24(min..max)
    }

    #[derive(Debug)]
    struct Dummy<T>(Box<(T, T)>);

    impl<T> Dummy<T> {
        fn new(a: T, b: T) -> Self {
            Self(Box::new((a, b)))
        }
    }

    impl<T: Clone> Clone for Dummy<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T: PartialEq> PartialEq for Dummy<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    proptest! {
        #[test]
        fn new(a in any::<[f32; 12]>()) {
            let array = Array::<f32, s!(2, 6)>::new(a);
            assert_eq!(array.data.0, a);
            assert_eq!(array.shape, [2, 6]);
            assert_eq!(array.shape(), &[2, 6]);

            let b = [Dummy::new(a[0], a[1]), Dummy::new(a[2], a[3]), Dummy::new(a[4], a[5]),
                     Dummy::new(a[6], a[7]), Dummy::new(a[8], a[9]), Dummy::new(a[10], a[11])];
            let c = b.clone();
            let array = Array::<Dummy<f32>, s!(2, 3)>::new(b);
            assert_eq!(array.data.0, c);
            assert_eq!(array.shape, [2, 3]);
            assert_eq!(array.shape(), &[2, 3]);
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
        fn add_scalar_0(arr in array32::<u32>(0, HALF_U32), b in 0..HALF_U32) {
            let array = Array::<u32, s!(3, 2, 4)>::from(&arr[..]);
            let c = &array + b;
            let d = array + b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn add_scalar_1(arr in array24::<u32>(0, HALF_U32), b in 0..HALF_U32) {
            let array: Array::<u32, s!(3, 2, 4)> = Array::new(arr);
            let c = &array + b;
            let d = array + b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn sub_scalar_0(arr in array32::<u32>(HALF_U32, u32::MAX), b in 0..HALF_U32) {
            let array = Array::<u32, s!(3, 6)>::from(&arr[..]);
            let c = &array - b;
            let d = array - b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn sub_scalar_1(arr in array24::<u32>(HALF_U32, u32::MAX), b in 0..HALF_U32) {
            let array = Array::<u32, s!(4, 6)>::from(&arr[..]);
            let c = &array - b;
            let d = array - b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn mul_scalar_0(arr in array32::<u32>(0, HALF_U32 / 2), b in 0..4u32) {
            let array = Array::<u32, s!(4, 5)>::from(&arr[..]);
            let c = &array * b;
            let d = array * b;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn scalar_add_0(arr in array32::<u32>(0, HALF_U32), b in 0..HALF_U32) {
            let arr = Array::<u32, s!(5, 5)>::from(&arr[..]);
            let c = b + &arr;
            let d = b + arr;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn scalar_add_1(arr in array16::<u16>(0, u16::MAX / 2), b in 0..u16::MAX/2) {
            let array = Array::<u16, s!(4, 4)>::new(arr);
            let c = b + &array;
            let d = b + array;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn scalar_sub_0(arr in array32::<u32>(0, HALF_U32), b in HALF_U32..u32::MAX) {
            let arr = Array::<u32, s!(4, 7)>::from(&arr[..]);
            let c = b - &arr;
            let d = b - arr;
            prop_assert_eq!(c, d);
        }

        #[test]
        fn scalar_mul(arr in array32::<u32>(u32::MIN, u32::MAX / 4), b in 0..4u32) {
            let array = Array::<u32, s!(2, 4, 3)>::from(&arr[..]);
            let c = b * &array;
            let d = &array * b;
            let e = b * array;
            prop_assert_eq!(c, d);
            prop_assert_eq!(d, e);
        }
    }

    #[test]
    fn nested() {
        type Packet = Array<f32, s!(4, 1)>;
        let p0 = Packet::new([1.0, 2.0, 3.0, 4.0]);
        let p1 = Packet::new([5.0, 6.0, 7.0, 8.0]);
        let a0 = Array::<Packet, s!(4, 1)>::new([p0, p1, p0, p1]);
        let sum0 = &a0 + 2.0;
        assert_eq!(
            sum0,
            Array::<Packet, s!(4, 1)>::new([
                Packet::new([3.0, 4.0, 5.0, 6.0]),
                Packet::new([7.0, 8.0, 9.0, 10.0]),
                Packet::new([3.0, 4.0, 5.0, 6.0]),
                Packet::new([7.0, 8.0, 9.0, 10.0]),
            ])
        );

        let sum1 = 2.0 + &a0;
        assert_eq!(sum1, sum0);
    }

    // Test that the array is using the correct constructor when
    // the data is copyable or cloneable.
    #[test]
    fn copy_clone_slice() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let _arr_0 = Array::<i32, s!(2, 5)>::from(&a[..]);
        let b = [
            Box::new(1),
            Box::new(2),
            Box::new(3),
            Box::new(4),
            Box::new(5),
            Box::new(6),
        ];
        let _arr_1 = Array::<Box<u32>, s!(2, 3)>::from(&b[..]);
    }
}
