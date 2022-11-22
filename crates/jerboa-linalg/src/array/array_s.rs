use crate::core::{ArrayCore, FixedShape, FixedSized, Data, Scalar, Shape};
use core::ops::{Deref};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, DerefMut};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S: FixedShape>(ArrayCore<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>)
where
    [(); <S as FixedShape>::N_ELEMS]:;

impl<A, S> Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayCore {
            data: FixedSized(data),
            shape: <S as FixedShape>::SHAPE,
            strides: <S as FixedShape>::STRIDES,
        })
    }

    pub fn from_slice(slice: &[A]) -> Self
        where A: Clone
    {
        let data: &[A; <S as FixedShape>::N_ELEMS] = slice.try_into().unwrap();
        Self::new(data.clone())
    }
}

impl<A, B, S> Add<B> for Array<A, S>
where
    A: Add<B, Output = A> + Clone,
    B: Scalar,
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    type Output = Self;

    fn add(self, rhs: B) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<A, S> Deref for Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    type Target = ArrayCore<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, S> DerefMut for Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A, S> From<&[A]> for Array<A, S>
    where A: Clone,
          S: FixedShape,
          [(); <S as FixedShape>::N_ELEMS]:,
{
    fn from(slice: &[A]) -> Self {
        // assert_eq!(slice.len(), <S as FixedShape>::N_ELEMS, "slice length must match array size");
        // let mut data: [A; <S as FixedShape>::N_ELEMS] = unsafe { core::mem::zeroed() };
        // data.clone_from_slice(slice);
        // Self::new(data)
        Self::from_slice(slice)
    }
}

impl<A, S> Debug for Array<A, S>
where S: FixedShape,
      <S as Shape>::UnderlyingType: Debug,
        [(); <S as FixedShape>::N_ELEMS]:,
      [(); <S as FixedShape>::N_ELEMS]:,
      A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Array")
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
    fn new() {
        let array = Array::<u32, cs!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.shape, [2, 5]);
        assert_eq!(array.shape(), &[2, 5]);
    }

    #[test]
    fn from_slice() {
        let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array = Array::<u32, cs!(2, 4)>::from(&a[..8]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn n_elems() {
        let array: Array<f32, cs!(3, 2, 4)> = Array::new([0.0; 24]);
        assert_eq!(array.n_elems(), 24);
    }
}
