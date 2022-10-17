use crate::{packet::Packet, traits::{DotProduct, Floating, Num, Sqrt}, Abs, Normalization};
use core::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
        SubAssign,
    },
};

pub struct Vector<T, const N: usize>(pub(crate) Packet<T, N>);

pub struct Point<T, const N: usize>(pub(crate) Packet<T, N>);

pub struct Normal<T, const N: usize>(pub(crate) Packet<T, N>);

// Macro to generate the implementations of Copy and Clone.
macro_rules! impl_vec_copy_clone {
    ($($vec:ident)*) => {
        $(
            impl<T, const N: usize> Clone for $vec<T, N>
                where T: Clone
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }

            impl<T, const N: usize> Copy for $vec<T, N> where T: Copy {}
        )*
    };
}

// Macro to generate the implementations of Debug and Display.
macro_rules! impl_vec_debug_display {
    ($($vec:ident)*) => {
        $(
            impl<T: Debug, const N: usize> Debug for $vec<T, N> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}({:?})", stringify!($vec), self.0)
                }
            }

            impl<T: Display, const N: usize> Display for $vec<T, N> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )*
    };
}

// Macro to generate the implementations of Index and IndexMut.
macro_rules! impl_vec_indexing {
    ($($vec:ident)*) => {
        $(
            impl<T, const N: usize> Index<usize> for $vec<T, N> {
                type Output = T;
                fn index(&self, index: usize) -> &Self::Output {
                    &self.0[index]
                }
            }

            impl<T, const N: usize> IndexMut<usize> for $vec<T, N> {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    &mut self.0[index]
                }
            }
        )*
    };
}

// Macro to generate the implementations of Default.
macro_rules! impl_vec_default {
    ($($vec:ident)*) => {
        $(
            impl<T, const N: usize> Default for $vec<T, N>
                where T: Default
            {
                fn default() -> Self {
                    Self(Array::default())
                }
            }
        )*
    };
}

// Macro to generate the implementations of Eq and PartialEq.
macro_rules! impl_vec_eq {
    ($($vec:ident)*) => {
        $(
            impl<T, const N: usize> PartialEq for $vec<T, N>
                where T: PartialEq
            {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }

            impl<T, const N: usize> Eq for $vec<T, N> where T: Eq, Self: PartialEq {}
        )*
    };
}

// Macro to generate the implementations of Neg.
macro_rules! impl_vec_neg {
    ($($vec:ident)*) => {
        $(
            impl<T: Num, const N: usize> Neg for $vec<T, N>
                where T: Neg<Output = T>
            {
                type Output = $vec<T, N>;
                fn neg(self) -> Self::Output {
                    $vec(-self.0)
                }
            }
        )*
    };
}

// Macro to generate the implementations of binary operators.
macro_rules! impl_vec_binary_op {
    (
        $vec_a:ident $({$op_trait:ident, $op:ident})|* $vec_b:ident = $vec_c:ident
    ) => {
        $(
            impl<T: Num, const N: usize> $op_trait<$vec_b<T, N>> for $vec_a<T, N> {
                type Output = $vec_c<T, N>;
                fn $op(self, rhs: $vec_b<T, N>) -> Self::Output {
                    $vec_c(self.0.$op(rhs.0))
                }
            }

            impl<'a, T: Num, const N: usize> $op_trait<&'a $vec_b<T, N>> for $vec_a<T, N> {
                type Output = $vec_c<T, N>;
                fn $op(self, rhs: &'a $vec_b<T, N>) -> Self::Output {
                    $vec_c(self.0.$op(rhs.0))
                }
            }

            impl<'a, T: Num, const N: usize> $op_trait<$vec_b<T, N>> for &'a $vec_a<T, N> {
                type Output = $vec_c<T, N>;
                fn $op(self, rhs: $vec_b<T, N>) -> Self::Output {
                    $vec_c(self.0.$op(rhs.0))
                }
            }

            impl<'a, 'b, T: Num, const N: usize> $op_trait<&'a $vec_b<T, N>> for &'b $vec_a<T, N> {
                type Output = $vec_c<T, N>;
                fn $op(self, rhs: &'a $vec_b<T, N>) -> Self::Output {
                    $vec_c(self.0.$op(rhs.0))
                }
            }
        )*
    };
    (
        asgmt $vec_a:ident $({$op_trait_assign:ident, $op_assign:ident})|* $vec_b:ident
    ) => {
        $(
            impl<T: Num, const N: usize> $op_trait_assign<$vec_b<T, N>> for $vec_a<T, N>
                where T: $op_trait_assign
            {
                fn $op_assign(&mut self, rhs: $vec_b<T, N>) {
                    self.0.$op_assign(rhs.0);
                }
            }

            impl<'a, T: Num, const N: usize> $op_trait_assign<&'a $vec_b<T, N>> for $vec_a<T, N>
                where T: $op_trait_assign
            {
                fn $op_assign(&mut self, rhs: &'a $vec_b<T, N>) {
                    self.0.$op_assign(rhs.0);
                }
            }
        )*
    };
}

// Macro to generate the implementations of binary operators of vectors with
// scalars.
macro_rules! impl_vec_element_op {
    ($vec:ident $({$op_trait:ident, $op:ident})|*) => {
        $(
            impl<T, const N: usize> $op_trait<T> for $vec<T, N>
                where T: $op_trait<Output = T> + Copy
            {
                type Output = $vec<T, N>;
                fn $op(self, rhs: T) -> Self::Output {
                    $vec(self.0.$op(rhs))
                }
            }

            impl<'a, T, const N: usize> $op_trait<&'a T> for $vec<T, N>
                where T: $op_trait<Output = T> + Copy
            {
                type Output = $vec<T, N>;
                fn $op(self, rhs: &'a T) -> Self::Output {
                    $vec(self.0.$op(*rhs))
                }
            }
        )*
    };
    (asgmt $vec_a:ident $({$op_trait_assign:ident, $op_assign:ident})|*) => {
        $(
            impl<T: Num, const N: usize> $op_trait_assign<T> for $vec_a<T, N>
                where T: $op_trait_assign<T>
            {
                fn $op_assign(&mut self, rhs: T) {
                    self.0.$op_assign(rhs);
                }
            }

            impl<'a, T: Num, const N: usize> $op_trait_assign<&'a T> for $vec_a<T, N>
                where for <'e> T: $op_trait_assign<&'e T>
            {
                fn $op_assign(&mut self, rhs: &'a T) {
                    self.0.$op_assign(rhs);
                }
            }
        )*
    };
}

// Macro to generate the shared methods of vector types.
macro_rules! impl_vec_common_methods {
    ($($vec:ident),* $(,)*) => {
        $(
            impl<T: Num, const N: usize> $vec<T, N> {
                pub const fn new(arr: [T; N]) -> Self {
                    $vec(Array::new(arr))
                }

                // todo: const fn
                pub fn zeros() -> Self {
                    $vec(Array::new([T::zero(); N]))
                }

                // todo: const fn
                pub fn ones() -> Self {
                    $vec(Array::new([T::one(); N]))
                }

                pub fn sqrt(&self) -> Self where T: Floating {
                    $vec(self.0.sqrt())
                }

                pub fn abs(&self) -> Self
                where T: Abs<Output = T>
                {
                    $vec(self.0.abs())
                }
            }
        )*
    };
}

// Macro to generate the implementations of conversion between vector types.
macro_rules! impl_vec_conversion {
    ($from:ident ==> $to:ident) => {
        impl<T, const N: usize> From<$from<T, N>> for $to<T, N> {
            fn from(v: $from<T, N>) -> Self {
                $to(v.0)
            }
        }

        impl<'a, T, const N: usize> From<&'a $from<T, N>> for $to<T, N>
        where
            T: Clone,
        {
            fn from(v: &'a $from<T, N>) -> Self {
                $to(v.0.clone())
            }
        }
    };
}

// Macro to generate the implementations of DotProduct.
macro_rules! impl_vec_dot_product {
    ($vec_a:ident <.> $vec_b:ident) => {
        impl<T: Num, const N: usize> DotProduct<$vec_b<T, N>> for $vec_a<T, N> {
            type Output = T;

            fn dot(self, rhs: $vec_b<T, N>) -> Self::Output {
                self.0.dot(rhs.0)
            }
        }

        impl<'a, T: Num, const N: usize> DotProduct<&'a $vec_b<T, N>> for $vec_a<T, N>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = T;

            fn dot(self, rhs: &'a $vec_b<T, N>) -> Self::Output {
                self.0.dot(rhs.0)
            }
        }

        impl<'a, 'b, T: Num, const N: usize> DotProduct<&'a $vec_b<T, N>> for &'b $vec_a<T, N>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = T;

            fn dot(self, rhs: &'a $vec_b<T, N>) -> Self::Output {
                self.0.dot(rhs.0)
            }
        }
    };
}

macro_rules! impl_scalar_vec_binary_op_ {
    ($vec:ident $({$op_trait:ident, $op:ident})|* $t:ty) => {
        $(
            impl<const N: usize> $op_trait<$vec<$t, N>> for $t {
                type Output = $vec<$t, N>;
                fn $op(self, rhs: $vec<$t, N>) -> Self::Output {
                    $vec(rhs.0.$op(self))
                }
            }
        )*
    };
}

macro_rules! impl_scalar_vec_binary_op {
    ($vec:ident $($t:ty)*) => {
        $(
            impl_scalar_vec_binary_op_!($vec {Mul, mul}|{Div, div} $t);
        )*
    };
}

impl_vec_copy_clone!(Vector Point Normal);
impl_vec_debug_display!(Vector Point Normal);
impl_vec_indexing!(Vector Point Normal);
impl_vec_default!(Vector Point Normal);
impl_vec_eq!(Vector Point Normal);
impl_vec_neg!(Vector Point Normal);

impl_vec_binary_op!(Normal {Add, add}|{Sub, sub} Vector = Vector);
impl_vec_binary_op!(Normal {Add, add}|{Sub, sub} Normal = Normal);

impl_vec_binary_op!(Point {Add, add}|{Sub, sub} Vector = Point);
impl_vec_binary_op!(asgmt Point {AddAssign, add_assign}|{SubAssign, sub_assign} Vector);

impl_vec_binary_op!(Vector {Add, add}|{Sub, sub}|{Mul, mul}|{Div, div}|{Rem, rem} Vector = Vector);
impl_vec_binary_op!(asgmt Vector {AddAssign, add_assign}|{SubAssign, sub_assign}|{MulAssign, mul_assign}|{DivAssign, div_assign}|{RemAssign, rem_assign} Vector);

impl_scalar_vec_binary_op!(Vector u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);
impl_scalar_vec_binary_op!(Point u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);
impl_scalar_vec_binary_op!(Normal u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);

impl_vec_element_op!(Vector {Mul, mul}|{Div, div}|{Rem, rem});
impl_vec_element_op!(asgmt Vector {MulAssign, mul_assign}|{DivAssign, div_assign}|{RemAssign, rem_assign});

impl_vec_common_methods!(Vector, Point, Normal);
impl_vec_conversion!(Vector ==> Point);
impl_vec_conversion!(Vector ==> Normal);
impl_vec_conversion!(Point ==> Vector);
impl_vec_conversion!(Normal ==> Vector);

impl_vec_dot_product!(Vector <.> Vector);
impl_vec_dot_product!(Vector <.> Normal);
impl_vec_dot_product!(Normal <.> Vector);
impl_vec_dot_product!(Normal <.> Normal);

impl<T: Num> Vector<T, 2> {
    pub fn cross(self, rhs: &Vector<T, 2>) -> T {
        self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0]
    }

    pub fn perp_dot(self, rhs: &Vector<T, 2>) -> T {
        self.0[0] * rhs.0[1] + self.0[1] * rhs.0[0]
    }
}

impl<T: Num> Vector<T, 3> {
    pub fn cross(self, rhs: &Vector<T, 3>) -> Vector<T, 3> {
        Vector(Packet::new([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ]))
    }

    pub fn normalize(self) -> Vector<T, 3>
    where
        T: Floating,
    {
        Self(self.0.normalize())
    }

    pub fn normalized(&mut self)
    where
        T: Floating,
    {
        self.0.normalized()
    }
}

#[inline(always)]
pub const fn pnt2<T: Num>(x: T, y: T) -> Point<T, 2> {
    Point::new([x, y])
}

#[inline(always)]
pub const fn pnt3<T: Num>(x: T, y: T, z: T) -> Point<T, 3> {
    Point::new([x, y, z])
}

#[inline(always)]
pub const fn vec2<T: Num>(x: T, y: T) -> Vector<T, 2> {
    Vector::new([x, y])
}

#[inline(always)]
pub const fn vec3<T: Num>(x: T, y: T, z: T) -> Vector<T, 3> {
    Vector::new([x, y, z])
}

#[inline(always)]
pub const fn vec4<T: Num>(x: T, y: T, z: T, w: T) -> Vector<T, 4> {
    Vector::new([x, y, z, w])
}

#[inline(always)]
pub const fn nml2<T: Num>(x: T, y: T) -> Normal<T, 2> {
    Normal::new([x, y])
}

#[inline(always)]
pub const fn nml3<T: Num>(x: T, y: T, z: T) -> Normal<T, 3> {
    Normal::new([x, y, z])
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn vector_mul() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(4.0, 5.0, 6.0);
        let v3 = v1 * v2;
        assert_eq!(v3, vec3(4.0, 10.0, 18.0));

        let v4 = v1 * 2.0;
        assert_eq!(v4, vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_scalar_mul() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2, vec3(2.0, 4.0, 6.0));
    }

    // #[test]
    // fn scalar_vector_mul() {
    //     let v1 = vec3(1.0, 2.0, 3.0);
    //     let v2 = 2.0 * v1;
    //     assert_eq!(v2, vec3(2.0, 4.0, 6.0));
    // }

    #[test]
    fn debug_display() {
        let vec = vec3(1.0, 2.0, 3.0);
        assert_eq!(format!("{:?}", vec), "Vector([1.0, 2.0, 3.0])");
        assert_eq!(format!("{}", vec), "[1, 2, 3]");

        let pnt = pnt3(1.0, 2.0, 3.0);
        assert_eq!(format!("{:?}", pnt), "Point([1.0, 2.0, 3.0])");
        assert_eq!(format!("{}", pnt), "[1, 2, 3]");

        let nml = nml3(1.0, 2.0, 3.0);
        assert_eq!(format!("{:?}", nml), "Normal([1.0, 2.0, 3.0])");
        assert_eq!(format!("{}", nml), "[1, 2, 3]");
    }

    #[test]
    fn indexing() {
        let vec = Vector::new([1, 2, 3, 4, 5, 6]);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[4], 5);
    }

    #[test]
    fn dot() {
        let vec = vec3(1.0, 2.0, 3.0);
        let vec2 = vec3(4.0, 5.0, 6.0);
        assert_eq!(vec.dot(&vec2), 32.0);
        assert_eq!(vec2.dot(vec), 32.0);
    }
}
