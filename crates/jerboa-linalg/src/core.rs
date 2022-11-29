mod arith;
mod data;
mod index;
mod sealed;
mod shape;

pub(crate) use arith::*;
pub use data::*;
pub(crate) use sealed::Sealed;
pub use shape::*;

use std::fmt::{Debug, Error, Formatter};

/// A n-dimensional array.
pub struct ArrayCore<D, S, L = RowMajor>
where
    D: DataRaw,
    L: TLayout,
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
    pub(crate) _marker: std::marker::PhantomData<(D, L, S)>,
}

impl<D, S, L> ArrayCore<D, S, L>
where
    D: DataRaw,
    L: TLayout,
    S: Shape,
{
    /// Returns the number of elements in the array.
    pub fn n_elems(&self) -> usize {
        if let Some(n) = S::N_ELEMS {
            n
        } else {
            calc_n_elems(self.shape.as_slice())
        }
    }

    /// Returns the number of dimensions of the array.
    pub fn n_dims(&self) -> usize {
        if let Some(n) = S::N_DIMS {
            n
        } else {
            self.shape.as_slice().len()
        }
    }

    /// Returns the shape of the array.
    pub fn shape(&self) -> &[usize] {
        self.shape.as_slice()
    }

    /// Returns the strides of the array.
    pub fn strides(&self) -> &[usize] {
        self.strides.as_slice()
    }

    /// Returns the layout of the array.
    pub fn layout(&self) -> Layout {
        L::LAYOUT
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

impl<D0, D1, S0, S1, L0, L1, E> PartialEq<ArrayCore<D1, S1, L1>> for ArrayCore<D0, S0, L0>
where
    D0: DataRaw<Elem = E>,
    D1: DataRaw<Elem = E>,
    E: PartialEq,
    L0: TLayout,
    L1: TLayout,
    S0: Shape,
    S1: Shape,
{
    fn eq(&self, other: &ArrayCore<D1, S1, L1>) -> bool {
        let have_same_layout = self.layout == other.layout;

        if have_same_layout {
            for (a, b) in self.shape.as_slice().iter().zip(other.shape.as_slice()) {
                if a != b {
                    return false;
                }
            }
        } else {
            for (a, b) in self
                .shape
                .as_slice()
                .iter()
                .zip(other.shape.as_slice().iter().rev())
            {
                if a != b {
                    return false;
                }
            }
        }

        unsafe {
            let a = self.data.as_ptr();
            let b = other.data.as_ptr();
            let n = self.n_elems();
            for i in 0..n {
                if *a.add(i) != *b.add(i) {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test_array_core_eq() {
    let a: ArrayCore<FixedSized<u32, 16>, s!(4, 4)> = ArrayCore {
        data: FixedSized([1; 16]),
        shape: <s!(4, 4) as Shape>::shape(),
        strides: <s!(4, 4) as Shape>::row_major_strides(),
        layout: Layout::RowMajor,
        _marker: Default::default(),
    };

    let b: ArrayCore<FixedSized<u32, 16>, s!(4, 4)> = ArrayCore {
        data: FixedSized([1; 16]),
        shape: <s!(4, 4) as Shape>::shape(),
        strides: <s!(4, 4) as Shape>::column_major_strides(),
        layout: Layout::ColumnMajor,
        _marker: Default::default(),
    };
    assert_eq!(a, b);

    let c: ArrayCore<FixedSized<u32, 12>, s!(4, 3)> = ArrayCore {
        data: FixedSized([1; 12]),
        shape: <s!(4, 3) as Shape>::shape(),
        strides: <s!(4, 3) as Shape>::row_major_strides(),
        layout: Layout::RowMajor,
        _marker: Default::default(),
    };

    let d: ArrayCore<FixedSized<u32, 12>, s!(3, 4)> = ArrayCore {
        data: FixedSized([1; 12]),
        shape: <s!(3, 4) as Shape>::shape(),
        strides: <s!(3, 4) as Shape>::column_major_strides(),
        layout: Layout::ColumnMajor,
        _marker: Default::default(),
    };

    assert_eq!(c, d);
    assert_ne!(a, c);
}
