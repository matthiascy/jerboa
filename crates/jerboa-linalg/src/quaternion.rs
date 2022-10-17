use crate::{Packet, Num};
use std::ops::{Index, IndexMut, Neg};

// TODO:
// - [ ] Add tests
// - [ ] Add documentation
// - [ ] Add examples
// - [ ] mul
// - [ ] div
// - [ ] rcp
// - [ ] sqr_norm
// - [ ] norm
// - [ ] dot
// - [ ] abs
// - [ ] exp
// - [ ] log
// - [ ] pow
// - [ ] sqrt
// - [ ] to matrix
// - [ ] from matrix
// - [ ] to euler
// - [ ] from euler
// - [ ] slerp
// - [ ] from axis angle
// - [ ] to axis angle
// - [ ] Display, Debug

/// Quaternion.
///
/// A quaternion is a four-dimensional hyper-complex number used in
/// three-dimensional rotation and orientation. It's represented in
/// the form of `ai + bj + ck + d`, where `a`, `b`, `c`, and `d` are
/// real numbers. The `i`, `j`, and `k` are imaginary basis vectors.
pub struct Quaternion<T: Num>(Packet<T, 4>);

impl<T: Num> Clone for Quaternion<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: Num> Copy for Quaternion<T> {}

impl<T: Num> PartialEq for Quaternion<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Num + Eq> Eq for Quaternion<T> {}

impl<T: Num> Index<usize> for Quaternion<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num> IndexMut<usize> for Quaternion<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Num> Quaternion<T> {
    pub fn new(arr: [T; 4]) -> Self {
        Self(Packet::new(arr))
    }

    pub fn identity() -> Self {
        Self::new([T::one(), T::zero(), T::zero(), T::one()])
    }

    pub fn real(&self) -> T {
        self.0[3]
    }

    pub fn imag(&self) -> Packet<T, 3> {
        Packet::new([self.0[0], self.0[1], self.0[2]])
    }
}

impl<T: Num + Neg<Output = T>> Quaternion<T> {
    pub fn conj(&self) -> Self {
        Self::new([-self.0[0], -self.0[1], -self.0[2], self.0[3]])
    }
}

#[inline(always)]
pub fn quat<T: Num>(x: T, y: T, z: T, w: T) -> Quaternion<T> {
    Quaternion::new([x, y, z, w])
}
