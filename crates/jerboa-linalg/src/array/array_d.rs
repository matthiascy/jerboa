use crate::core::{ArrayCore, DynSized, FixedShape};

/// Fix-sized array on the heap.
#[repr(transparent)]
pub struct ArrayD<A, S: FixedShape>(ArrayCore<DynSized<A>, S>);

impl<A, S: FixedShape> ArrayD<A, S>
    where
        [(); <S as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayCore {
            data: DynSized(Vec::from(data)),
            shape: S::value(),
            strides: S::strides(),
        })
    }
}
