use crate::core::{ArrayCore, CShape, FixedSized, Layout, RowMajor, Scalar, TLayout};
use core::ops::Deref;
use std::{
    fmt::{Debug, Formatter},
    ops::{Add, DerefMut},
};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S, L = RowMajor>(ArrayCore<FixedSized<A, { <S as CShape>::N_ELEMS }>, S, L>)
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
    pub fn new(data: [A; <S as CShape>::N_ELEMS]) -> Self {
        let strides = match L::LAYOUT {
            Layout::RowMajor => <S as CShape>::ROW_MAJOR_STRIDES,
            Layout::ColumnMajor => <S as CShape>::COLUMN_MAJOR_STRIDES,
        };
        Self(ArrayCore {
            data: FixedSized(data),
            shape: <S as CShape>::SHAPE,
            strides,
            // layout: Layout::RowMajor,
            layout: L::LAYOUT,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn from_slice(slice: &[A]) -> Self
    where
        A: Clone,
    {
        let data: &[A; <S as CShape>::N_ELEMS] = slice.try_into().unwrap();
        Self::new(data.clone())
    }
}

impl<A, B, S, L> Add<B> for Array<A, S, L>
where
    A: Add<B, Output = A> + Clone,
    B: Scalar,
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    type Output = Self;

    fn add(self, rhs: B) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<A, S, L> Deref for Array<A, S, L>
where
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    type Target = ArrayCore<FixedSized<A, { <S as CShape>::N_ELEMS }>, S, L>;

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

impl<A, S, L> From<&[A]> for Array<A, S, L>
where
    A: Clone,
    L: TLayout,
    S: CShape,
    [(); <S as CShape>::N_ELEMS]:,
{
    fn from(slice: &[A]) -> Self {
        // assert_eq!(slice.len(), <S as CShape>::N_ELEMS, "slice length must match
        // array size"); let mut data: [A; <S as CShape>::N_ELEMS] = unsafe
        // { core::mem::zeroed() }; data.clone_from_slice(slice);
        // Self::new(data)
        Self::from_slice(slice)
    }
}

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
    use crate::core::s;

    #[test]
    fn new() {
        let array = Array::<u32, s!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.shape, [2, 5]);
        assert_eq!(array.shape(), &[2, 5]);
    }

    #[test]
    fn from_slice() {
        let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let array = Array::<u32, s!(2, 4)>::from(&a[..8]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn n_elems() {
        let array: Array<f32, s!(3, 2, 4)> = Array::new([0.0; 24]);
        assert_eq!(array.n_elems(), 24);
    }
}
