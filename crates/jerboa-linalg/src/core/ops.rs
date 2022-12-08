use crate::core::{ArrCore, DataClone, DataRawMut, Decay, Shape, TLayout, DataMut};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait Scalar: Clone {}

macro impl_scalar($($t:ty),*) {
    $(
        impl Scalar for $t {}
    )*
}

impl_scalar!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

// todo:
//  + neg
//  + array & array ops with broadcasting
//  + approx for floating numbers
//  + add examples in doc for each implmentation

/// Macro for implementing binary ops for array and scalar.
///
/// Implementations try to avoid unnecessary allocations by
/// reusing the consumed array if possible.
macro impl_arr_core_scalar_binary_op($tr:ident, $op:tt, $mth:ident, $doc:expr) {
    /// Performs element-wise
    #[doc=$doc]
    /// between an array and a scalar.
    /// The array is the left-hand side operand, the scalar is the right-hand side operand.
    /// The scalar is broadcasted to the shape of the array.
    ///
    /// # Note
    ///
    /// This method is not really consuming the array, but resuse data storage.
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
                    // The value is consumed, we don't need to clone it. As for dropping, the
                    // corresponding value in array will be overwritten by new value, so it's
                    // fine to leave the `elem` automatically dropped at the end of the loop;
                    // it won't be dropped twice.
                    let elem = core::ptr::read(array.data.as_ptr().add(i));
                    core::ptr::write(array.data.as_mut_ptr().add(i), elem $op rhs.clone());
                }
            }
            array
        }
    }

    /// Performs element-wise
    #[doc=$doc]
    /// between a reference of an array and a scalar.
    /// The reference is the left-hand side operand, the scalar is the right-hand side operand.
    ///
    /// # Note
    ///
    /// This method does not cloning the array storage but requires the array to be
    /// cloneable. Under the hood, the an uninitialized data storage is allocated and
    /// the result is written to it.
    impl<'a, A, B, D, S, L> $tr<B> for &'a ArrCore<D, S, L>
    where A: $tr<B, Output = A> + Clone,
          B: Scalar,
          D: DataRawMut<Elem = A>,
          <D as Decay>::Type: DataRawMut<Elem = A>,
          L: TLayout,
          S: Shape,
    {
        type Output = ArrCore<<D as Decay>::Type, S, L>;

        fn $mth(self, rhs: B) -> Self::Output {
            let n_elems = self.n_elems();
            let mut data = unsafe { D::alloc_uninit(n_elems) };
            let mut i = 0;
            while i < n_elems {
                unsafe {
                    // We wan't to keep the original array untouched, so we need to clone the
                    // element. Besides, the element is directly read from the original memory,
                    // in other words, it's a copy of the original value, so we don't want it
                    // to be dropped, otherwise the original value will be dropped.
                    let elem = core::ptr::read(self.data.as_ptr().add(i));
                    core::ptr::write(data.as_mut_ptr().add(i), elem.clone() $op rhs.clone());
                    core::mem::forget(elem);
                }
                i += 1;
            }
            ArrCore {
                data,
                shape: self.shape.clone(),
                strides: self.strides.clone(),
                layout: self.layout.clone(),
                marker: core::marker::PhantomData,
            }
        }
    }
}

impl_arr_core_scalar_binary_op!(Add, +, add, "addition");
impl_arr_core_scalar_binary_op!(Sub, -, sub, "subtraction");
impl_arr_core_scalar_binary_op!(Mul, *, mul, "multiplication");
impl_arr_core_scalar_binary_op!(Div, /, div, "division");
impl_arr_core_scalar_binary_op!(Rem, %, rem, "remainder");
impl_arr_core_scalar_binary_op!(BitAnd, &, bitand, "bitwise AND");
impl_arr_core_scalar_binary_op!(BitOr, |, bitor, "bitwise OR");
impl_arr_core_scalar_binary_op!(BitXor, ^, bitxor, "bitwise XOR");
impl_arr_core_scalar_binary_op!(Shl, <<, shl, "left shift");
impl_arr_core_scalar_binary_op!(Shr, >>, shr, "right shift");

impl<A, D, S, L> Add<ArrCore<D, S, L>> for ArrCore<D, S, L>
where
    A: Add<A, Output = A>,
    D: DataRawMut<Elem = A>,
    L: TLayout,
    S: Shape,
{
    type Output = ArrCore<D, S, L>;

    fn add(self, rhs: ArrCore<D, S, L>) -> Self::Output {
        let mut array = self;
        let n_elems = array.n_elems();
        for i in 0..n_elems {
            unsafe {
                let elem = core::ptr::read(array.data.as_ptr().add(i));
                let rhs_elem = core::ptr::read(rhs.data.as_ptr().add(i));
                core::ptr::write(array.data.as_mut_ptr().add(i), elem + rhs_elem);
            }
        }
        array
    }
}

impl<'a, A, D, S, L> Add<&'a ArrCore<D, S, L>> for ArrCore<D, S, L>
where
    A: Add<A, Output = A>,
    D: DataRawMut<Elem = A>,
    L: TLayout,
    S: Shape,
{
    type Output = ArrCore<D, S, L>;

    fn add(self, rhs: &'a ArrCore<D, S, L>) -> Self::Output {
        let mut array = self;
        let n_elems = array.n_elems();
        for i in 0..n_elems {
            unsafe {
                let elem = core::ptr::read(array.data.as_ptr().add(i));
                let rhs_elem = core::ptr::read(rhs.data.as_ptr().add(i));
                core::ptr::write(array.data.as_mut_ptr().add(i), elem + rhs_elem);
            }
        }
        array
    }
}

impl<'a, A, D, S, L> Add<ArrCore<D, S, L>> for &'a ArrCore<D, S, L>
    where A: Add<A, Output = A> + Clone,
          D: DataRawMut<Elem = A>,
          L: TLayout,
          S: Shape,
{
    type Output = ArrCore<D, S, L>;

    fn add(self, rhs: ArrCore<D, S, L>) -> Self::Output {
        let mut array = rhs;
        let n_elems = array.n_elems();
        for i in 0..n_elems {
            unsafe {
                let rhs_elem = core::ptr::read(array.data.as_ptr().add(i));
                let elem = core::ptr::read(self.data.as_ptr().add(i));
                core::ptr::write(array.data.as_mut_ptr().add(i), elem.clone() + rhs_elem);
            }
        }
        array
    }
}

impl<'a, 'b, A, D, S, L> Add<&'b ArrCore<D, S, L>> for &'a ArrCore<D, S, L>
where
    A: Add<A, Output = A> + Clone,
    D: DataRawMut<Elem = A>,
    <D as Decay>::Type: DataRawMut<Elem = A>,
    L: TLayout,
    S: Shape,
{
    type Output = ArrCore<<D as Decay>::Type, S, L>;

    fn add(self, rhs: &'b ArrCore<D, S, L>) -> Self::Output {
        let n_elems = self.n_elems();
        let mut data = unsafe { D::alloc_uninit(n_elems) };
        let mut i = 0;
        while i < n_elems {
            unsafe {
                let lhs_elem = core::ptr::read(self.data.as_ptr().add(i));
                let rhs_elem = core::ptr::read(rhs.data.as_ptr().add(i));
                core::ptr::write(
                    data.as_mut_ptr().add(i),
                    lhs_elem.clone() + rhs_elem.clone(),
                );
            }
            i += 1;
        }
        ArrCore {
            data,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            layout: self.layout.clone(),
            marker: core::marker::PhantomData,
        }
    }
}

macro impl_arr_core_scalar_binary_assign_op($tr_assgn:ident, $tr:ident, $tr_op:tt, $mth:ident, $doc:expr) {
    /// Performs element-wise
    #[doc=$doc]
    /// between an array and a scalar.
    /// The array is the left-hand side operand, the scalar is the right-hand side operand.
    /// The scalar is broadcasted to the shape of the array.
    ///
    /// # Note
    ///
    /// This method is not really consuming the array, but resuse data storage.
    impl<A, B, D, S, L> $tr_assgn<B> for ArrCore<D, S, L>
    where
        A: $tr<B, Output = A> + Clone,
        B: Scalar,
        D: DataRawMut<Elem = A>,
        L: TLayout,
        S: Shape,
    {
        fn $mth(&mut self, rhs: B) {
            let n_elems = self.n_elems();
            for i in 0..n_elems {
                unsafe {
                    let elem = core::ptr::read(self.data.as_ptr().add(i));
                    core::ptr::write(self.data.as_mut_ptr().add(i), elem $tr_op rhs.clone());
                }
            }
        }
    }
}

impl_arr_core_scalar_binary_assign_op!(AddAssign, Add, +, add_assign, "addition assignment");
impl_arr_core_scalar_binary_assign_op!(SubAssign, Sub, -, sub_assign, "subtraction assignment");
impl_arr_core_scalar_binary_assign_op!(MulAssign, Mul, *, mul_assign, "multiplication assignment");
impl_arr_core_scalar_binary_assign_op!(DivAssign, Div, /, div_assign, "division assignment");
impl_arr_core_scalar_binary_assign_op!(RemAssign, Rem, %, rem_assign, "remainder assignment");
impl_arr_core_scalar_binary_assign_op!(BitAndAssign, BitAnd, &, bitand_assign, "bitwise AND assignment");
impl_arr_core_scalar_binary_assign_op!(BitOrAssign, BitOr, |, bitor_assign, "bitwise OR assignment");
impl_arr_core_scalar_binary_assign_op!(BitXorAssign, BitXor, ^, bitxor_assign, "bitwise XOR assignment");
impl_arr_core_scalar_binary_assign_op!(ShlAssign, Shl, <<, shl_assign, "left shift assignment");
impl_arr_core_scalar_binary_assign_op!(ShrAssign, Shr, >>, shr_assign, "right shift assignment");

/// Macro for implementing binary ops for arrays where the LHS is a scalar.
///
/// Implementations try to avoid unnecessary allocations by reusing the consumed
/// array if possible.
///
/// # Implementation details
///
/// There is a distinction between commutative and non-commutative
/// implementations. For commutative operations, the return value is created
/// using uninitialized memory and then then written by the calculation result,
/// rather than being cloned from RHS. For non-commutative operations, the
/// return value is cloned from the RHS and then the computation result is
/// written to it.
macro impl_arr_core_scalar_lhs_op {
    (commutative $t:ty, $tr:ident, $op:tt, $mth:ident, $doc:expr) => {
        /// Performs element-wise
        #[doc=$doc]
        /// between a scalar and an array.
        /// The scalar is left-hand side operand, the array is the right-hand side operand.
        ///
        /// # Note
        ///
        /// This method is not really consuming the array, but resuse data storage.
        impl<A, D, S, L> $tr<ArrCore<D, S, L>> for $t
        where
            A: $tr<$t, Output = A>,
            D: DataRawMut<Elem = A>,
            L: TLayout,
            S: Shape,
        {
            type Output = ArrCore<D, S, L>;

            fn $mth(self, rhs: ArrCore<D, S, L>) -> Self::Output {
                let mut array = rhs;
                let n_elems = array.n_elems();
                for i in 0..n_elems {
                    unsafe {
                        let elem = core::ptr::read(array.data.as_ptr().add(i));
                        core::ptr::write(array.data.as_mut_ptr().add(i), elem $op self.clone());
                    }
                }
                array
            }
        }

        /// Performs element-wise
        #[doc=$doc]
        /// between a scalar and a reference of an array.
        /// The reference is the right-hand side operand, the scalar is the left-hand side operand.
        ///
        /// # Note
        ///
        /// This method does not cloning the array storage but requires the array to be
        /// cloneable. Under the hood, the an uninitialized data storage is allocated and
        /// the result is written to it.
        impl<'a, A, D, S, L> $tr<&'a ArrCore<D, S, L>> for $t
        where
            for<'e> &'e A: $tr<$t, Output = A>,
            D: DataRawMut<Elem = A>,
            <D as Decay>::Type: DataRawMut<Elem = A>,
            L: TLayout,
            S: Shape,
        {
            type Output = ArrCore<<D as Decay>::Type, S, L>;

            fn $mth(self, rhs: &'a ArrCore<D, S, L>) -> Self::Output {
                let n_elems = rhs.n_elems();
                let mut data = unsafe { D::alloc_uninit(n_elems) };
                let mut i = 0;
                while i < n_elems {
                    unsafe {
                        let elem = core::ptr::read(rhs.data.as_ptr().add(i));
                        core::ptr::write(data.as_mut_ptr().add(i), &elem $op self);
                    }
                    i += 1;
                }
                ArrCore {
                    data,
                    shape: rhs.shape.clone(),
                    strides: rhs.strides.clone(),
                    layout: rhs.layout.clone(),
                    marker: core::marker::PhantomData,
                }
            }
        }
    },
    (non-commutative $t:ty, $tr:ident, $op:tt, $mth:ident, $doc:expr) => {
        /// Performs element-wise
        #[doc=$doc]
        /// between a scalar and an array.
        /// The scalar is left-hand side operand, the array is the right-hand side operand.
        ///
        /// # Note
        ///
        /// This method is not really consuming the array, but resuse data storage.
        impl<D, S, L> $tr<ArrCore<D, S, L>> for $t
        where
            D: DataRawMut<Elem = $t>,
            L: TLayout,
            S: Shape,
        {
            type Output = ArrCore<D, S, L>;

            fn $mth(self, rhs: ArrCore<D, S, L>) -> Self::Output {
                let mut array = rhs;
                let n_elems = array.n_elems();
                for i in 0..n_elems {
                    unsafe {
                        let elem = core::ptr::read(array.data.as_ptr().add(i));
                        core::ptr::write(array.data.as_mut_ptr().add(i), self $op elem);
                    }
                }
                array
            }
        }

        /// Performs element-wise
        #[doc=$doc]
        /// between a scalar and a reference of an array.
        /// The reference is the right-hand side operand, the scalar is the left-hand side operand.
        ///
        /// # Note
        ///
        /// This method does not cloning the array storage but requires the array to be
        /// cloneable. Under the hood, the an uninitialized data storage is allocated and
        /// the result is written to it.
        impl<'a, D, S, L> $tr<&'a ArrCore<D, S, L>> for $t
        where
            D: DataClone<Elem = $t>,
            L: TLayout,
            S: Shape,
        {
            type Output = ArrCore<D, S, L>;

            fn $mth(self, rhs: &'a ArrCore<D, S, L>) -> Self::Output {
                let mut data = rhs.data.clone();
                let mut i = 0;
                while i < rhs.n_elems() {
                    unsafe {
                        let elem = core::ptr::read(data.as_ptr().add(i));
                        core::ptr::write(data.as_mut_ptr().add(i), self $op elem);
                    }
                    i += 1;
                }
                ArrCore {
                    data,
                    shape: rhs.shape.clone(),
                    strides: rhs.strides.clone(),
                    layout: rhs.layout.clone(),
                    marker: core::marker::PhantomData,
                }
            }
        }
    }
}

macro impl_arr_core_integer_lhs_op($($t:ty),*) {
    $(
        impl_arr_core_scalar_lhs_op!(commutative $t, Add, +, add, "addition");
        impl_arr_core_scalar_lhs_op!(commutative $t, Mul, *, mul, "multiplication");
        impl_arr_core_scalar_lhs_op!(commutative $t, BitAnd, &, bitand, "bitwise AND");
        impl_arr_core_scalar_lhs_op!(commutative $t, BitOr, |, bitor, "bitwise OR");
        impl_arr_core_scalar_lhs_op!(commutative $t, BitXor, ^, bitxor, "bitwise XOR");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Sub, -, sub, "subtraction");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Div, /, div, "division");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Rem, %, rem, "remainder");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Shl, <<, shl, "left shift");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Shr, >>, shr, "right shift");
    )*
}

macro impl_arr_core_float_lhs_op($($t:ty),*) {
    $(
        impl_arr_core_scalar_lhs_op!(commutative $t, Add, +, add, "addition");
        impl_arr_core_scalar_lhs_op!(commutative $t, Mul, *, mul, "multiplication");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Sub, -, sub, "subtraction");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Div, /, div, "division");
        impl_arr_core_scalar_lhs_op!(non-commutative $t, Rem, %, rem, "remainder");
    )*
}

impl_arr_core_float_lhs_op!(f32, f64);
impl_arr_core_scalar_lhs_op!(commutative bool, BitAnd, &, bitand, "bitwise AND");
impl_arr_core_scalar_lhs_op!(commutative bool, BitOr, |, bitor, "bitwise OR");
impl_arr_core_scalar_lhs_op!(commutative bool, BitXor, ^, bitxor, "bitwise XOR");
impl_arr_core_integer_lhs_op!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize);

macro impl_arr_core_unary_op($tr:ident, $mth:ident, $op:tt) {
    impl<A, D, S, L> $tr for ArrCore<D, S, L>
    where
        A: $tr<Output = A>,
        D: DataRawMut<Elem = A>,
        L: TLayout,
        S: Shape,
    {
        type Output = ArrCore<D, S, L>;

        fn $mth(self) -> Self::Output {
            let mut array = self;
            let n_elems = array.n_elems();
            for i in 0..n_elems {
                unsafe {
                    let elem = core::ptr::read(array.data.as_ptr().add(i));
                    core::ptr::write(array.data.as_mut_ptr().add(i), $op elem);
                }
            }
            array
        }
    }

    impl<'a, A, D, S, L> $tr for &'a ArrCore<D, S, L>
    where
        A: $tr<Output = A> + Clone,
        D: DataClone<Elem = A>,
        L: TLayout,
        S: Shape,
    {
        type Output = ArrCore<D, S, L>;

        fn $mth(self) -> Self::Output {
            let mut data = self.data.clone();
            let mut i = 0;
            while i < self.n_elems() {
                unsafe {
                    let elem = core::ptr::read(data.as_ptr().add(i));
                    core::ptr::write(data.as_mut_ptr().add(i), $op elem);
                }
                i += 1;
            }
            ArrCore {
                data,
                shape: self.shape.clone(),
                strides: self.strides.clone(),
                layout: self.layout.clone(),
                marker: core::marker::PhantomData,
            }
        }
    }
}

impl_arr_core_unary_op!(Neg, neg, -);
impl_arr_core_unary_op!(Not, not, !);
