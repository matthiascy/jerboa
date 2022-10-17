use std::ops::{Deref, DerefMut};

#[repr(C, align(16))]
pub struct Align16<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align16<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align16<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align16<T, N> {
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        Self(slice_to_array(slice))
    }

    pub const fn from_slice_unchecked(slice: &[T]) -> Self {
        Self(slice_to_array_unchecked(slice))
    }
}

#[repr(C, align(32))]
pub struct Align32<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align32<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align32<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align32<T, N> {
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        Self(slice_to_array(slice))
    }

    pub const fn from_slice_unchecked(slice: &[T]) -> Self {
        Self(slice_to_array_unchecked(slice))
    }
}

#[repr(C, align(64))]
pub struct Align64<T: Copy, const N: usize>(pub(crate) [T; N]);

impl<T: Copy, const N: usize> Deref for Align64<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const N: usize> DerefMut for Align64<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy, const N: usize> Align64<T, N> {
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    pub const fn from_slice(slice: &[T]) -> Self {
        Self(slice_to_array(slice))
    }

    pub const fn from_slice_unchecked(slice: &[T]) -> Self {
        Self(slice_to_array_unchecked(slice))
    }
}

/// Converts a slice of `T` into an `[T, N]`, where `N` is the length of the
/// slice. This function does not check that the length of the slice is equal to
/// `N`. In case the length is less than `N`, the remaining elements are filled
/// with the first element of the slice.
pub const fn slice_to_array_unchecked<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    // todo: create zeroed packet
    // todo: get rid of Copy trait
    let mut array = [slice[0]; N];
    if slice.len() >= N {
        let mut i = 0;
        while i < N {
            array[i] = slice[i];
            i += 1;
        }
    } else {
        let mut i = 0;
        while i < slice.len() {
            array[i] = slice[i];
            i += 1;
        }
    }
    array
}

/// Converts a slice into an packet with length checked.
pub const fn slice_to_array<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    assert!(
        slice.len() >= N,
        "slice length must be at least the number of lanes"
    );
    // todo: create zeroed packet
    // todo: get rid of Copy trait
    let mut array = [slice[0]; N];

    let mut i = 0;
    while i < N {
        array[i] = slice[i];
        i += 1;
    }
    array
}
