use std::fmt::Write;
use crate::core::sealed::Sealed;

// Data is always stored contiguously in memory, ordering only affects how the
// data is interpreted (i.e. the shape and strides).
// ? Is it possible to only have a single Data trait and use a generic parameter ?

pub mod dyn_sized;
pub mod fixed_sized;

/// Trait providing raw access to the elements of the storage, implemented by
/// all storage types.
pub unsafe trait DataRaw: Sized + Sealed {
    /// The type of the elements stored in the storage.
    type Elem;

    /// Get a pointer to the first element of the storage.
    fn as_ptr(&self) -> *const Self::Elem;
}

/// Trait providing slice access to the elements of the storage, implemented by
/// all storage types.
pub unsafe trait Data: DataRaw {
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
