mod shape;
mod storage;
mod sealed;

pub(crate) use shape::*;
pub(crate) use storage::*;

/// A n-dimensional array.
pub struct ArrayBase<D, S>
where
    D: Storage,
    S: Shape,
{
    /// The data of the array.
    pub(crate) data: D,
    /// The shape of the array including the number of dimensions and the size
    /// of each dimension.
    pub(crate) shape: S::Type,
}

impl<D, S> ArrayBase<D, S>
    where
        D: Storage,
        S: Shape,
{
    pub fn shape(&self) -> &S::Type {
        &self.shape
    }
}