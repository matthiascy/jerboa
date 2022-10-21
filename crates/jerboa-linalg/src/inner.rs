mod dyn_sized;
mod fixed_sized;
mod sealed;
mod shape;
mod storage;
mod index;

pub(crate) use dyn_sized::*;
pub(crate) use fixed_sized::*;
pub(crate) use sealed::Sealed;
pub(crate) use shape::*;
pub use shape::cs;
pub(crate) use storage::*;

/// A n-dimensional array.
pub struct ArrayInner<D, S>
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

impl<D, S> ArrayInner<D, S>
where
    D: Storage,
    S: Shape,
{
    pub fn new(data: D, shape: S::Type) -> Self {
        Self { data, shape }
    }

    pub fn shape(&self) -> &S::Type {
        &self.shape
    }
}