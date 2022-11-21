mod sealed;
mod shape;
mod storage;
mod index;

pub(crate) use storage::dyn_sized::*;
pub(crate) use storage::fixed_sized::*;
pub(crate) use sealed::Sealed;
pub(crate) use shape::*;
pub(crate) use storage::*;

/// A n-dimensional array.
pub struct ArrayCore<D, S>
where
    D: Storage,
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
    D: Storage,
    S: Shape,
{
    pub fn shape(&self) -> &S::UnderlyingType {
        &self.shape
    }
    pub fn strides(&self) -> &S::UnderlyingType {
        &self.strides
    }
}

impl<D, S> ArrayCore<D, S>
where D: Storage,
      S: FixedShape
{
    pub fn n_elems(&self) -> usize {
        S::N_ELEMS
    }
}

impl<D> ArrayCore<D, ShapeDyn>
    where D: Storage,
{
    pub fn n_elems(&self) -> usize {
        self.shape.iter().product()
    }
}

// View: offset, shape, strides, data