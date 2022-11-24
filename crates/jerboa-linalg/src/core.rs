mod arith;
mod data;
mod index;
mod sealed;
mod shape;

pub(crate) use arith::*;
pub(crate) use sealed::Sealed;
pub use shape::*;
pub use data::*;

use std::fmt::{Debug, Error, Formatter};

/// A n-dimensional array.
pub struct ArrayCore<D, S, L: TLayout = RowMajor>
where
    D: DataRaw,
    S: Shape,
{
    /// The data of the array.
    pub(crate) data: D,

    /// The shape of the array including the number of dimensions and the size
    /// of each dimension.
    pub(crate) shape: S::UnderlyingType,

    /// The number of elements needed to skip to get to the next element along
    /// each dimension. Its interpretation depends on the layout of the array.
    pub(crate) strides: S::UnderlyingType,

    /// The memory layout of the array.
    pub(crate) layout: Layout,

    /// The marker for the layout.
    pub(crate) _marker: std::marker::PhantomData<L>,
}

impl<D, S, L> ArrayCore<D, S, L>
where
    D: DataRaw,
    L: TLayout,
    S: Shape,
{
    pub fn shape(&self) -> &[usize] {
        self.shape.as_slice()
    }

    pub fn strides(&self) -> &[usize] {
        self.strides.as_slice()
    }

    pub fn n_dims(&self) -> usize {
        self.shape.as_slice().len()
    }

    pub fn layout(&self) -> Layout {
        L::LAYOUT
    }
}

impl<D, S, L> ArrayCore<D, S, L>
where
    D: DataRaw,
    L: TLayout,
    S: CShape,
{
    pub fn n_elems(&self) -> usize {
        S::N_ELEMS
    }
}

impl<D, L> ArrayCore<D, DynamicShape, L>
where
    D: DataRaw,
    L: TLayout,
{
    pub fn n_elems(&self) -> usize {
        calc_n_elems(&self.shape)
    }
}

impl<D, S, L> Debug for ArrayCore<D, S, L>
where
    D: DataRaw + Debug,
    L: TLayout,
    S: Shape,
    S::UnderlyingType: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ArrayCore")
            .field("data", &self.data)
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .field("layout", &self.layout)
            .finish()
    }
}
