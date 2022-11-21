use crate::core::{display_slice, RawStorage, RawStorageMut, Sealed, Storage, StorageMut};
use core::fmt::{Debug, Display, Formatter};

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

impl<A: PartialEq, const N: usize> PartialEq for FixedSized<A, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<A: PartialEq + Eq, const N: usize> Eq for FixedSized<A, N> {}

unsafe impl<A, const N: usize> RawStorage for FixedSized<A, N> {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.0.as_ptr()
    }
}

unsafe impl<A, const N: usize> Storage for FixedSized<A, N> {
    fn as_slice(&self) -> &[Self::Elem] {
        &self.0
    }
}

unsafe impl<A, const N: usize> RawStorageMut for FixedSized<A, N> {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.0.as_mut_ptr()
    }
}

unsafe impl<A, const N: usize> StorageMut for FixedSized<A, N> {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        &mut self.0
    }
}

impl<A: Debug, const N: usize> Debug for FixedSized<A, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FixedSized({:?})", self.0))
    }
}

impl<A: Display, const N: usize> Display for FixedSized<A, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        display_slice(f, &self.0)
    }
}

impl<A, const N: usize> FixedSized<A, N> {
    pub fn as_slice(&self) -> &[A] {
        <Self as Storage>::as_slice(self)
    }

    pub fn as_mut_slice(&mut self) -> &mut [A] {
        <Self as StorageMut>::as_mut_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_display() {
        let a = FixedSized([1, 2, 3]);
        assert_eq!(format!("{:?}", a), "FixedSized([1, 2, 3])");
        assert_eq!(format!("{}", a), "[1, 2, 3]");
    }
}
