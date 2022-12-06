use crate::core::Sealed;

// Data is always stored contiguously in memory, ordering only affects how the
// data is interpreted (i.e. the shape and strides).
// ? Is it possible to only have a single Data trait and use a generic parameter
// ?

pub mod dyn_sized;
pub mod fixed_sized;

pub use dyn_sized::DynSized;
pub use fixed_sized::FixedSized;

/// Memory layout of the data.
///
/// + row-major layout (or C layout): the data is stored row by row in memory;
///   the strides grow from right to left; the last dimension varies the
/// fastest.
///
/// + col(umn)-major layout (or Fortran layout): the data is stored column by
///   column in memory; the strides grow from left to right; the first dimension
///   varies the fastest.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Layout {
    RowMajor,
    ColumnMajor,
}

// !!! workarounds for incomplete feature: adt_const_params
pub trait TLayout: Sized + Copy + Clone {
    const LAYOUT: Layout;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RowMajor;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ColumnMajor;

impl TLayout for RowMajor {
    const LAYOUT: Layout = Layout::RowMajor;
}

impl TLayout for ColumnMajor {
    const LAYOUT: Layout = Layout::ColumnMajor;
}

/// Trait to obtain the decayed type of a type.
/// This is used to remove references and constness from a type.
#[const_trait]
pub trait Decay {
    type Type;
}

/// Trait providing raw access to the elements of the storage, implemented by
/// all storage types.
pub unsafe trait DataRaw: Sized + Decay + Sealed {
    /// The type of the elements stored in the storage.
    type Elem;

    /// Get a pointer to the first element of the storage.
    fn as_ptr(&self) -> *const Self::Elem;

    /// Allocate a new storage given required capacity.
    ///
    /// The storage may be allocated on the heap or on the stack depending
    /// on the concrete data container. The required size of the storage must
    /// match the shape of array.
    ///
    /// Unsafe because the caller must ensure that the storage is properly
    /// initialized after the call. Then, the storage will be dropped
    /// automatically when it goes out of scope.
    ///
    /// # Safety
    ///
    /// The storage is uninitialized [`MaybeUninit`]. The caller must initialize
    /// it before using it. Please use `ptr::write` or `ptr::copy` to
    /// initialize the storage elements without dropping the uninitialized
    /// values.
    unsafe fn alloc_uninit(len: usize) -> <Self as Decay>::Type;
}

/// Trait providing slice access to the elements of the storage, implemented by
/// all storage types.
pub unsafe trait Data: DataRaw {
    /// Get a slice of the elements of the storage.
    fn as_slice(&self) -> &[Self::Elem];
}

/// Trait providing mutable raw access to the elements of the storage,
/// implemented by storage types that can provide mutable access to their
/// elements.
pub unsafe trait DataRawMut: DataRaw {
    /// Get a mutable pointer to the first element of the storage.
    fn as_mut_ptr(&mut self) -> *mut Self::Elem;
}

/// Trait providing mutable slice access to the elements of the storage,
/// implemented by storage types that can provide mutable access to their
/// elements.
pub unsafe trait DataMut: Data + DataRawMut {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

pub trait DataClone: Data + DataMut + Clone {}

impl<T: Clone> DataClone for T where T: Data + DataMut {}

pub trait DataCopy: DataClone + Copy {}

impl<T: Copy> DataCopy for T where T: DataClone {}

pub(crate) fn display_slice<T: core::fmt::Display>(
    f: &mut core::fmt::Formatter<'_>,
    seq: &[T],
) -> core::fmt::Result {
    f.write_str("[")?;
    let last = seq.len() - 1;
    for (i, x) in seq.iter().enumerate() {
        if i == last {
            write!(f, "{}", x)?;
        } else {
            write!(f, "{}, ", x)?;
        }
    }
    f.write_str("]")
}
