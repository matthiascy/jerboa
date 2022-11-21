use crate::inner::{ArrayInner, DynSized, Shape, ShapeDyn};

/// Dynamic-sized array on the heap.
#[repr(transparent)]
pub struct ArrayDyn<A>(ArrayInner<DynSized<A>, ShapeDyn>);

impl<A> ArrayDyn<A> {
    /// Creates a new array.
    pub fn new() -> Self {
        Self(ArrayInner {
            data: DynSized(Vec::new()),
            shape: ShapeDyn::value(),
            strides: ShapeDyn::strides(),
        })
    }
}
