use crate::core::{ArrayCore, DynSized, Scalar, Shape, ShapeDyn};
use core::{
    fmt::{Debug, Error, Formatter},
    ops::*,
};

/// Dynamic-sized array on the heap.
#[repr(transparent)]
pub struct ArrayDyn<A>(ArrayCore<DynSized<A>, ShapeDyn>);

impl<A> ArrayDyn<A> {
    /// Creates a new empty array.
    pub fn new(shape: &[usize]) -> Self {
        Self::empty(shape)
    }

    /// Creates an empty array.
    pub fn empty(shape: &[usize]) -> Self {
        let shape: <ShapeDyn as Shape>::UnderlyingType = shape.to_vec();
        let mut strides = shape.clone();
        ShapeDyn::calc_strides(&shape, &mut strides);
        Self(ArrayCore {
            data: DynSized(Vec::with_capacity(ShapeDyn::calc_n_elems(&shape))),
            shape,
            strides,
        })
    }

    /// Creates a new array from a vector.
    pub fn from_vec(shape: &[usize], data: Vec<A>) -> Self {
        assert_eq!(
            ShapeDyn::calc_n_elems(shape),
            data.len(),
            "data length must match array size"
        );
        let shape: <ShapeDyn as Shape>::UnderlyingType = shape.to_vec();
        let mut strides = shape.clone();
        ShapeDyn::calc_strides(&shape, &mut strides);
        Self(ArrayCore {
            data: DynSized(data),
            shape,
            strides,
        })
    }

    /// Creates a new array from a slice.
    pub fn from_slice(shape: &[usize], data: &[A]) -> Self
    where
        A: Clone,
    {
        assert_eq!(
            ShapeDyn::calc_n_elems(shape),
            data.len(),
            "data length must match array size"
        );
        let shape: <ShapeDyn as Shape>::UnderlyingType = shape.to_vec();
        let mut strides = shape.clone();
        ShapeDyn::calc_strides(&shape, &mut strides);
        Self(ArrayCore {
            data: DynSized(data.to_vec()),
            shape,
            strides,
        })
    }

    pub fn n_elems(&self) -> usize {
        ShapeDyn::calc_n_elems(&self.shape)
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }
}

impl<A> Debug for ArrayDyn<A>
where
    A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ArrayDyn")
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .field("data", &self.data)
            .finish()
    }
}

impl<A> Deref for ArrayDyn<A> {
    type Target = ArrayCore<DynSized<A>, ShapeDyn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> DerefMut for ArrayDyn<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro impl_array_dyn_binary_op($tr:ident, $mth:ident) {
    impl<A, B> $tr<B> for ArrayDyn<A>
    where
        A: $tr<B, Output = A> + Clone,
        B: Scalar,
    {
        type Output = Self;

        fn $mth(self, rhs: B) -> Self::Output {
            Self(self.0.$mth(rhs))
        }
    }
}

impl_array_dyn_binary_op!(Add, add);
impl_array_dyn_binary_op!(Sub, sub);
impl_array_dyn_binary_op!(Mul, mul);
impl_array_dyn_binary_op!(Div, div);
impl_array_dyn_binary_op!(Rem, rem);
impl_array_dyn_binary_op!(BitAnd, bitand);
impl_array_dyn_binary_op!(BitOr, bitor);
impl_array_dyn_binary_op!(BitXor, bitxor);
impl_array_dyn_binary_op!(Shl, shl);
impl_array_dyn_binary_op!(Shr, shr);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_elems() {
        let array: ArrayDyn<f32> = ArrayDyn::empty(&[3, 2, 2]);
        println!("{:?}", array);
        assert_eq!(array.n_elems(), 12);
    }
}
