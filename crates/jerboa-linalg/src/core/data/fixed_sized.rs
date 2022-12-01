use crate::core::{display_slice, Data, DataMut, DataRaw, DataRawMut, Sealed};
use core::fmt::{Debug, Display, Formatter};

/// Fixed-sized array storage.
#[repr(transparent)]
pub struct FixedSized<A, const N: usize>(pub(crate) [A; N]);

impl<A, const N: usize> Sealed for FixedSized<A, N> {}
impl<'a, A, const N: usize> Sealed for &'a FixedSized<A, N> {}
impl<'a, A, const N: usize> Sealed for &'a mut FixedSized<A, N> {}

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

unsafe impl<A, const N: usize> DataRaw for FixedSized<A, N> {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.0.as_ptr()
    }
}

unsafe impl<'a, A, const N: usize> DataRaw for &'a FixedSized<A, N> {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.0.as_ptr()
    }
}

unsafe impl<'a, A, const N: usize> DataRaw for &'a mut FixedSized<A, N> {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.0.as_ptr()
    }
}

unsafe impl<A, const N: usize> Data for FixedSized<A, N> {
    fn as_slice(&self) -> &[Self::Elem] {
        &self.0
    }
}

unsafe impl<'a, A, const N: usize> Data for &'a FixedSized<A, N> {
    fn as_slice(&self) -> &[Self::Elem] {
        &self.0
    }
}

unsafe impl<'a, A, const N: usize> Data for &'a mut FixedSized<A, N> {
    fn as_slice(&self) -> &[Self::Elem] {
        &self.0
    }
}

unsafe impl<A, const N: usize> DataRawMut for FixedSized<A, N> {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.0.as_mut_ptr()
    }
}

unsafe impl<'a, A, const N: usize> DataRawMut for &'a mut FixedSized<A, N> {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.0.as_mut_ptr()
    }
}

unsafe impl<A, const N: usize> DataMut for FixedSized<A, N> {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        &mut self.0
    }
}

unsafe impl<'a, A, const N: usize> DataMut for &'a mut FixedSized<A, N> {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        &mut self.0
    }
}

impl<A: Debug, const N: usize> Debug for FixedSized<A, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
        <Self as Data>::as_slice(self)
    }

    pub fn as_mut_slice(&mut self) -> &mut [A] {
        <Self as DataMut>::as_mut_slice(self)
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
