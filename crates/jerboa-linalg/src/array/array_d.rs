use std::fmt::{Debug, Formatter};
use std::ops::{Add, Deref, DerefMut};
use crate::Array;
use crate::core::{ArrayCore, DynSized, FixedShape, Scalar, Shape};

/// Fix-sized array on the heap.
#[repr(transparent)]
pub struct ArrayD<A, S: FixedShape>(ArrayCore<DynSized<A>, S>);

impl<A, S: FixedShape> ArrayD<A, S>
    where
        [(); <S as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayCore {
            data: DynSized(Vec::from(data)),
            shape: <S as FixedShape>::SHAPE,
            strides: <S as FixedShape>::STRIDES,
        })
    }
}

impl<A, S> Deref for ArrayD<A, S>
    where
        S: FixedShape,
        [(); <S as FixedShape>::N_ELEMS]:,
{
    type Target = ArrayCore<DynSized<A>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, S> DerefMut for ArrayD<A, S>
    where
        S: FixedShape,
        [(); <S as FixedShape>::N_ELEMS]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A, B, S> Add<B> for ArrayD<A, S>
    where A: Add<B, Output = A> + Clone,
          B: Scalar,
          S: FixedShape,
          [(); <S as FixedShape>::N_ELEMS]:,
{
    type Output = Self;

    fn add(self, rhs: B) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<A, S> Debug for ArrayD<A, S>
    where S: FixedShape,
          <S as Shape>::UnderlyingType: Debug,
          [(); <S as FixedShape>::N_ELEMS]:,
          A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayD")
            .field("data", &self.data)
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    use crate::core::cs;
    use super::*;

    #[test]
    fn n_elems() {
        let array: ArrayD<f32, cs!(3, 2, 4)> = ArrayD::new([0.0; 24]);
        assert_eq!(array.n_elems(), 24);
    }
}