use std::fmt::Debug;

pub trait Dimension: Debug + Clone + PartialEq + Eq + Default + Sync + Send {
    /// The number of dimensions. None for dynamic dimensions.
    const N_DIMS: Option<usize>;

    /// The number of elements. None for dynamic dimensions.
    const N_ELEMENTS: Option<usize>;

    /// The number of dimensions.
    fn n_dims(&self) -> usize;

    /// The number of elements.
    fn n_elements(&self) -> usize;
}

pub trait IntoDimension {
    type Dim: Dimension;

    fn into_dimension(self) -> Self::Dim;
}

impl<D: Dimension> IntoDimension for D {
    type Dim = D;

    fn into_dimension(self) -> Self::Dim {
        self
    }
}

/// Array dimension.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DimBase<I: ?Sized>(I);

impl<I> DimBase<I> {
    pub(crate) fn new(index: I) -> DimBase<I> {
        DimBase(index)
    }

    pub(crate) fn index(&self) -> &I {
        &self.0
    }
}

/// Fixed dimension.
pub type Dim<const N: usize> = DimBase<[usize; N]>;

/// Dynamic dimension.
pub type DimDyn = DimBase<Vec<usize>>;
