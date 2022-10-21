use crate::inner::sealed::Sealed;

/// The trait providing raw access to the elements of the storage.
pub unsafe trait RawStorage: Sized + Sealed {
    /// The type of the elements stored in the storage.
    type Item;

    /// Get a pointer to the first element of the storage.
    fn as_ptr(&self) -> *const Self::Item;
}

pub unsafe trait Storage: RawStorage {
    fn as_slice(&self) -> &[Self::Item];
}

/// The trait providing mutable raw access to the elements of the storage.
pub unsafe trait RawStorageMut: RawStorage {
    /// Get a mutable pointer to the first element of the storage.
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
}

pub unsafe trait StorageMut: Storage + RawStorageMut {
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

/// Fixed-sized array storage.
#[repr(transparent)]
pub struct FixedSized<A, const N: usize>(pub(crate) [A; N]);

impl<A, const N: usize> Sealed for FixedSized<A, N> {}

impl<A: Clone, const N: usize> Clone for FixedSized<A, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Copy, const N: usize> Copy for FixedSized<A, N> {}

unsafe impl<A, const N: usize> RawStorage for FixedSized<A, N> {
    type Item = A;

    fn as_ptr(&self) -> *const Self::Item {
        self.0.as_ptr()
    }
}

unsafe impl<A, const N: usize> Storage for FixedSized<A, N> {
    fn as_slice(&self) -> &[Self::Item] {
        &self.0
    }
}

unsafe impl<A, const N: usize> RawStorageMut for FixedSized<A, N> {
    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.0.as_mut_ptr()
    }
}

unsafe impl<A, const N: usize> StorageMut for FixedSized<A, N> {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        &mut self.0
    }
}

/// Dynamically-sized array storage.
pub struct DynSized<A>(pub(crate) Vec<A>);

impl<A> Sealed for DynSized<A> {}

impl<A: Clone> Clone for DynSized<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

unsafe impl<A> RawStorage for DynSized<A> {
    type Item = A;

    fn as_ptr(&self) -> *const Self::Item {
        self.0.as_ptr()
    }
}

unsafe impl<A> Storage for DynSized<A> {
    fn as_slice(&self) -> &[Self::Item] {
        &self.0
    }
}

unsafe impl<A> RawStorageMut for DynSized<A> {
    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.0.as_mut_ptr()
    }
}

unsafe impl<A> StorageMut for DynSized<A> {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        &mut self.0
    }
}
