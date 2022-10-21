use crate::inner::{display_slice, RawStorage, RawStorageMut, Sealed, Storage, StorageMut};
use core::fmt::{Debug, Display, Formatter};
use core::{ops::Deref};

/// Dynamically-sized array storage.
pub struct DynSized<A>(pub(crate) Vec<A>);

impl<A> Sealed for DynSized<A> {}

impl<A: Clone> Clone for DynSized<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: PartialEq> PartialEq for DynSized<A> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<A: PartialEq + Eq> Eq for DynSized<A> {}

unsafe impl<A> RawStorage for DynSized<A> {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.0.as_ptr()
    }
}

unsafe impl<A> Storage for DynSized<A> {
    fn as_slice(&self) -> &[Self::Elem] {
        &self.0
    }
}

unsafe impl<A> RawStorageMut for DynSized<A> {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.0.as_mut_ptr()
    }
}

unsafe impl<A> StorageMut for DynSized<A> {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        &mut self.0
    }
}

impl<A: Debug> Debug for DynSized<A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("DynSized({:?})", self.0))
    }
}

impl<A: Display> Display for DynSized<A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        display_slice(f, &self.0)
    }
}

impl<A> Deref for DynSized<A> {
    type Target = Vec<A>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_display() {
        let storage = DynSized(vec![1, 2, 3]);
        assert_eq!(format!("{:?}", storage), "DynSized([1, 2, 3])");
        assert_eq!(format!("{}", storage), "[1, 2, 3]");
    }
}
