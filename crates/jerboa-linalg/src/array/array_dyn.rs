use crate::core::{ArrayCore, DynSized, Shape, ShapeDyn};

/// Dynamic-sized array on the heap.
#[repr(transparent)]
pub struct ArrayDyn<A>(ArrayCore<DynSized<A>, ShapeDyn>);

impl<A> ArrayDyn<A> {
    /// Creates a new array.
    pub fn new(shape: &[usize]) -> Self {
        Self(ArrayCore {
            data: DynSized(Vec::new()),
            shape: ShapeDyn::value(),
            strides: ShapeDyn::strides(),
        })
    }
}
