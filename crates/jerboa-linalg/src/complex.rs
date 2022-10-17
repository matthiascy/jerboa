use crate::{Packet, Floating, Num};
use std::ops::{Mul, Neg};

pub struct Complex<T: Num>(Packet<T, 2>);

impl<T: Num> Complex<T> {
    pub fn new(arr: [T; 2]) -> Self {
        Self(Packet::new(arr))
    }

    pub fn real(&self) -> T {
        self.0[0]
    }

    pub fn imag(&self) -> T {
        self.0[1]
    }

    pub fn identity() -> Self {
        Self::new([T::one(), T::zero()])
    }

    pub fn sqr_norm(&self) -> T
    where
        for<'a> &'a T: ~const Mul<&'a T, Output = T>,
    {
        self.0.sqr_norm()
    }
}

impl<T: Num + Neg<Output = T>> Complex<T> {
    pub fn conj(&self) -> Self {
        Self::new([self.0[0], -self.0[1]])
    }
}

impl<T: Floating> Complex<T> {
    pub fn norm(&self) -> T
    where
        for<'a> &'a T: Mul<&'a T, Output = T>,
    {
        self.sqr_norm().sqrt()
    }
}
