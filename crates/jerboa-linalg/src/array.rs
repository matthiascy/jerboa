// mod dim;
//
// pub use dim::Dim;

pub struct Array<A, S> {
    data: Vec<A>,
    dim: S,
    strides: S,
}
