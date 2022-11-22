mod sealed;
mod shape;
mod data;
mod index;
mod arith;

use std::fmt::{Debug, Display, Error, Formatter};
pub(crate) use data::dyn_sized::*;
pub(crate) use data::fixed_sized::*;
pub(crate) use sealed::Sealed;
pub(crate) use shape::*;
pub(crate) use data::*;
pub(crate) use arith::*;

/// A n-dimensional array.
pub struct ArrayCore<D, S>
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
    /// each dimension.
    pub(crate) strides: S::UnderlyingType,
}

impl<D, S> ArrayCore<D, S>
where
    D: DataRaw,
    S: Shape,
{
    pub fn shape(&self) -> &S::UnderlyingType {
        &self.shape
    }

    pub fn strides(&self) -> &S::UnderlyingType {
        &self.strides
    }

    pub fn n_dims(&self) -> usize {
        self.shape.as_slice().len()
    }
}

impl<D, S> ArrayCore<D, S>
where D: DataRaw,
      S: FixedShape
{
    pub fn n_elems(&self) -> usize {
        S::N_ELEMS
    }
}

impl<D> ArrayCore<D, ShapeDyn>
    where D: DataRaw,
{
    pub fn n_elems(&self) -> usize {
        ShapeDyn::calc_n_elems(&self.shape)
    }
}

impl<D, S> Debug for ArrayCore<D, S>
    where D: DataRaw + Debug,
          S: Shape,
          S::UnderlyingType: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ArrayCore")
            .field("data", &self.data)
            .field("shape", &self.shape)
            .field("strides", &self.strides)
            .finish()
    }
}

// View: offset, shape, strides, data