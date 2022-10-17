use crate::{num::Num, packet::Packet};

impl<T: Num, const N: usize> IntoIterator for Packet<T, N> {
    type Item = T;
    type IntoIter = ::core::array::IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T: Num, const N: usize> IntoIterator for &'a Packet<T, N> {
    type Item = &'a T;
    type IntoIter = ::core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T: Num, const N: usize> IntoIterator for &'a mut Packet<T, N> {
    type Item = &'a mut T;
    type IntoIter = ::core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T: Num, const N: usize> Packet<T, N> {
    /// Returns an immutable iterator over the packet.
    pub fn iter(&'_ self) -> ::core::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T: Num, const N: usize> Packet<T, N> {
    /// Returns an mutable iterator over the packet.
    pub fn iter_mut(&'_ mut self) -> ::core::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}
