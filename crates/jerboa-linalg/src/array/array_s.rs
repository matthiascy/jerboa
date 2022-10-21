use crate::inner::{ArrayInner, FixedShape, FixedSized};
use core::ops::{Deref};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S: FixedShape>(ArrayInner<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>)
where
    [(); <S as FixedShape>::N_ELEMS]:;

impl<A, S> Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayInner {
            data: FixedSized(data),
            shape: S::shape(),
        })
    }
}

impl<A, S> Deref for Array<A, S>
where
    S: FixedShape,
    [(); <S as FixedShape>::N_ELEMS]:,
{
    type Target = ArrayInner<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::cs;
    use super::*;

    #[test]
    fn new() {
        let array = Array::<u32, cs!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(array.shape, [2, 5]);
        assert_eq!(array.shape(), &[2, 5]);
    }
}
