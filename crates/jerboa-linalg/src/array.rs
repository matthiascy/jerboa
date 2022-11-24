mod array_d;
mod array_dyn;
mod array_s;
mod packet;

pub trait NdArray {
    type Elem;
    fn shape(&self) -> &[usize];
    fn strides(&self) -> &[usize];
    fn n_elems(&self) -> usize;
    fn n_dims(&self) -> usize;
}

pub use array_d::ArrayD;
pub use array_dyn::ArrayDyn;
pub use array_s::Array;
pub use packet::Packet;
