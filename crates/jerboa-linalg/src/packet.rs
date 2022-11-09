mod arith;
mod iter;
mod ops;

use crate::num::{Num, Zero};
use std::{
    fmt::{Debug, Display, Write},
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[const_trait]
pub trait PacketSize {
    const SIZE: usize;

    fn size(&self) -> usize {
        Self::SIZE
    }
}

/// Packet of `N` elements of type `T`.
pub struct Packet<T, const N: usize> {
    pub(crate) data: [T; N],
    _marker: PhantomData<T>,
}

impl<T, const N: usize> Deref for Packet<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const N: usize> DerefMut for Packet<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, const N: usize> Index<usize> for Packet<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Packet<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: PartialEq, const N: usize> PartialEq for Packet<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: PartialEq + Eq, const N: usize> Eq for Packet<T, N> {}

impl<T: Debug, const N: usize> Debug for Packet<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (i, x) in self.data.iter().enumerate() {
            if i == self.data.len() - 1 {
                write!(f, "{:?}", x)?;
            } else {
                write!(f, "{:?}, ", x)?;
            }
        }
        f.write_char(']')?;
        Ok(())
    }
}

impl<T: Display, const N: usize> Display for Packet<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (i, x) in self.data.iter().enumerate() {
            if i == self.data.len() - 1 {
                write!(f, "{}", x)?;
            } else {
                write!(f, "{}, ", x)?;
            }
        }
        f.write_char(']')?;
        Ok(())
    }
}

impl<T, const N: usize> Default for Packet<T, N> {
    fn default() -> Self {
        Self {
            data: unsafe { core::mem::zeroed::<[T; N]>() },
            _marker: Default::default(),
        }
    }
}

impl<'a, T, const N: usize> Packet<T, N> {
    pub const fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn from_slice(slice: &'a [T]) -> Self
    where
        [T; N]: TryFrom<&'a [T]>,
        <[T; N] as TryFrom<&'a [T]>>::Error: Debug,
    {
        assert_eq!(slice.len(), N);
        Self {
            data: slice[0..N].try_into().unwrap(),
            _marker: PhantomData,
        }
    }

    pub fn from_mut_slice(slice: &'a mut [T]) -> Self
    where
        [T; N]: TryFrom<&'a [T]>,
        <[T; N] as TryFrom<&'a [T]>>::Error: Debug,
    {
        assert_eq!(slice.len(), N);
        Self {
            data: slice[0..N].try_into().unwrap(),
            _marker: PhantomData,
        }
    }

    pub fn from_exact_iter<I>(iter: I) -> Self
    where
        I: ExactSizeIterator<Item = T>,
    {
        assert_eq!(iter.len(), N);
        let mut data = unsafe { core::mem::zeroed::<[T; N]>() };
        for (i, x) in iter.enumerate() {
            data[i] = x;
        }
        Self {
            data,
            _marker: PhantomData,
        }
    }

    pub const fn len(&self) -> usize {
        N
    }

    pub fn map<F: FnMut(T) -> U, U: Num>(self, f: F) -> Packet<U, N> {
        Packet::new(self.data.map(f))
    }

    /// Returns an immutable reference to the first element in the packet.
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns an mutable pointer to the first element in the packet.
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

impl<'a, T, const N: usize> Packet<T, N> {
    pub fn clone_from_slice(slice: &'a [T]) -> Self
    where
        [T; N]: TryFrom<&'a [T]>,
        <[T; N] as TryFrom<&'a [T]>>::Error: Debug,
    {
        assert_eq!(slice.len(), N);
        Self {
            data: slice[0..N].try_into().unwrap(),
            _marker: PhantomData,
        }
    }
}

impl<T: Num, const N: usize> Zero for Packet<T, N> {
    fn zero() -> Self {
        Packet::new([T::zero(); N])
    }

    fn is_zero(&self) -> bool {
        for i in 0..N {
            if !self.data[i].is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T, const N: usize> Packet<T, N> {
    pub const fn new(data: [T; N]) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod packet_tests {
    use crate::packet::Packet;

    #[test]
    fn display() {
        let a = Packet::new([1, 2, 3, 4]);
        assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
    }

    #[test]
    fn debug() {
        let a = Packet::new([1, 2, 3, 4]);
        assert_eq!(format!("{:?}", a), "[1, 2, 3, 4]");
    }

    #[test]
    fn indexing() {
        let mut a = Packet::new([1.0, 2.0, 3.0, 4.0]);
        a[0] = 2.1;
        a[2] = 4.3;
        assert_eq!(a[0], 2.1);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 4.3);
        assert_eq!(a[3], 4.0);
    }

    #[test]
    fn from_slice() {
        let a: Packet<f32, 4> = Packet::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 7.0][0..4]);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
        assert_eq!(a[3], 4.0);
    }

    #[test]
    fn map() {
        let a = Packet::new([1.0; 12]);
        let b = a.map(|x| x * 2.0);
        for x in b.iter() {
            assert_eq!(x, &2.0);
        }
    }
}
