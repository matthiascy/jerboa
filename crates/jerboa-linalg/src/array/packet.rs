use crate::{
    array::array_s::Array,
    core::{CShape, ConstShape},
};
use crate::core::{RowMajor, TLayout};

/// Fix-sized one-dimension array on the stack.
#[repr(transparent)]
pub struct Packet<A, const N: usize, L: TLayout = RowMajor>(Array<A, ConstShape<(), N>, L>)
where
    [(); <ConstShape<(), N> as CShape>::N_ELEMS]:;

impl<A, const N: usize, L: TLayout> Packet<A, N, L>
where
    [(); <ConstShape<(), N> as CShape>::N_ELEMS]:,
{
    /// Creates a new array.
    pub fn new(data: [A; <ConstShape<(), N> as CShape>::N_ELEMS]) -> Self {
        Self(Array::new(data))
    }
}
