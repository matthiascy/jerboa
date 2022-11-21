use crate::core::{ArrayCore, FixedShape, FixedSized};
use core::ops::{Deref};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S: FixedShape>(ArrayCore<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>)
where
    [(); <S as FixedShape>::N_ELEMS]:;

impl<A, S> Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayCore {
            data: FixedSized(data),
            shape: S::value(),
            strides: S::strides(),
        })
    }
}

impl<A, S> Deref for Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    type Target = ArrayCore<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cs;
    use super::*;

    #[test]
    fn new() {
        let array = Array::<u32, cs!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.shape, [2, 5]);
        assert_eq!(array.shape(), &[2, 5]);
    }
}
