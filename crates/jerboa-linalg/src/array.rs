use std::ops::Deref;
use crate::inner::{DynSized, Shape, ShapeConst, ShapeDyn};
use super::inner::{ArrayBase, FixedShape, FixedSized};

/// Fix-sized array on the stack.
#[repr(transparent)]
pub struct Array<A, S: FixedShape>(ArrayBase<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>)
where
    [(); <S as FixedShape>::N_ELEMS]:;

impl<A, S> Array<A, S>
    where
        S: FixedShape,
        [(); <S as FixedShape>::N_ELEMS]:
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayBase {
            data: FixedSized(data),
            shape: S::shape(),
        })
    }
}

impl<A, S> Deref for Array<A, S>
    where
        S: FixedShape,
        [(); <S as FixedShape>::N_ELEMS]:
{
    type Target = ArrayBase<FixedSized<A, { <S as FixedShape>::N_ELEMS }>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Fix-sized one-dimension array on the stack.
#[repr(transparent)]
pub struct Packet<A, const N: usize>(Array<A, ShapeConst<(), N>>)
    where
        [(); <ShapeConst<(), N> as FixedShape>::N_ELEMS]:;

impl<A, const N: usize> Packet<A, N>
    where
        [(); <ShapeConst<(), N> as FixedShape>::N_ELEMS]:
{
    /// Creates a new array.
    pub fn new(data: [A; <ShapeConst<(), N> as FixedShape>::N_ELEMS]) -> Self {
        Self(Array::new(data))
    }
}

/// Fix-sized array on the heap.
#[repr(transparent)]
pub struct ArrayD<A, S: FixedShape>(ArrayBase<DynSized<A>, S>);

/// Dynamic-sized array on the heap.
#[repr(transparent)]
pub struct ArrayDyn<A>(ArrayBase<DynSized<A>, ShapeDyn>);

impl<A, S: FixedShape> ArrayD<A, S>
    where [(); <S as FixedShape>::N_ELEMS]:
{
    /// Creates a new array.
    pub fn new(data: [A; <S as FixedShape>::N_ELEMS]) -> Self {
        Self(ArrayBase {
            data: DynSized(Vec::from(data)),
            shape: S::shape(),
        })
    }
}

impl<A> ArrayDyn<A> {
    /// Creates a new array.
    pub fn new() -> Self {
        Self(ArrayBase {
            data: DynSized(Vec::new()),
            shape: ShapeDyn::shape(),
        })
    }
}

#[test]
fn array_new() {
    use crate::inner::cs;
    let array = Array::<u32, cs!(2, 5)>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(array.0.data.0, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(array.0.shape, [2, 5]);
    println!("{:?}", array.shape());
}
