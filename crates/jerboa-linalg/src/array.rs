// mod dim;
//
// pub use dim::Dim;

pub struct Array<M, S>
{
    data: M,
    dim: S,
    strides: S,
}
