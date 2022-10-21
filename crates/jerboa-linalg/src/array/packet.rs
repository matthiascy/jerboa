use crate::array::array_s::Array;
use crate::inner::{FixedShape, ShapeConst};

/// Fix-sized one-dimension array on the stack.
#[repr(transparent)]
pub struct Packet<A, const N: usize>(Array<A, ShapeConst<(), N>>)
where
    [(); <ShapeConst<(), N> as FixedShape>::N_ELEMS]:;

impl<A, const N: usize> Packet<A, N>
where
    [(); <ShapeConst<(), N> as FixedShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <ShapeConst<(), N> as FixedShape>::N_ELEMS]) -> Self {
        Self(Array::new(data))
    }
}
