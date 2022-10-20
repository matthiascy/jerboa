use core::marker::PhantomData;

pub trait ArrayData {

}

pub trait ArrayShape {
    const SIZE: usize;
}

pub trait ArrayShapeConst {
}

pub struct ArrayInner<D, S>
    where D: ArrayData,
          S: ArrayShape,
{
    data: D,
    _marker: PhantomData<(D, S)>,
}

/// Shape of a multi-dimensional array.
struct ShapeInner<I: ?Sized>(I);

/// Shape of a multi-dimensional array.
/// `N` is the number of dimensions.
pub type Shape<const DIM: usize> = ShapeInner<[usize; DIM]>;
pub type ShapeDyn = ShapeInner<Vec<usize>>;
pub struct ShapeConst<A, const N: usize>(PhantomData<[T; N]>);

pub struct ShapeC<A, const N: usize>(PhantomData<[T; N]>);

impl<const N: usize> ShapeC<(), N> {
}

impl<A, const N: usize> ShapeC<A, N> {
    pub fn shape() -> Vec<usize> {

    }
}

/// Shape of a multi-dimensional array with a known size and allocated on the stack.

/// Fixed-sized array storage.
pub struct FixSized<A, const N: usize>(pub(crate) [A; N]);

/// Dynamic-sized array storage.
pub struct DynSized<A>(pub(crate) Vec<A>);

/// Fix-sized array on the stack.
pub struct Array<A, S: ArrayShapeConst>(ArrayInner<FixSized<A, {S::SIZE}>, S>);

/// Fix-sized one-dimension array on the stack.
pub struct Packet<A, const N: usize>(Array<A, ShapeConst<(), N>>);

/// Fix-sized array on the heap.
pub struct ArrayD<A, S: ArrayShape>(ArrayInner<DynSized<A>, S>);

/// Dynamic-sized array on the heap.
pub struct ArrayDyn<A>(ArrayInner<DynSized<A>, ShapeDyn>);