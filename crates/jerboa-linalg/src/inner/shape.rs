use std::marker::PhantomData;

/// Trait for array shape.
pub trait Shape {
    /// Underlying shape storage type.
    type Type;

    /// Returns the value of storage type.
    fn shape() -> Self::Type;
}

/// Trait for fixed-sized multi-dimensional array with a known number of
/// dimensions and a known size for each dimension.
#[const_trait]
pub trait FixedShape: Shape {
    /// Number of dimensions in the multi-dimensional array.
    const N_DIMS: usize;

    /// Number of elements in the multi-dimensional array.
    const N_ELEMS: usize;

    /// Number of dimensions in the multi-dimensional array.
    fn size() -> usize {
        Self::N_ELEMS
    }

    /// Number of dimensions in the multi-dimensional array.
    fn n_elems() -> usize {
        Self::N_ELEMS
    }

    /// Number of dimensions in the multi-dimensional array.
    fn n_dims() -> usize {
        Self::N_DIMS
    }
}

impl Shape for () {
    type Type = ();

    fn shape() -> Self::Type {
        ()
    }
}

impl FixedShape for () {
    const N_DIMS: usize = 0;
    const N_ELEMS: usize = 0;
}

/// Array's shape type embedding directly the number of dimensions and the size
/// of each dimension.
pub struct ShapeConst<A: FixedShape, const N: usize>(PhantomData<[A; N]>);

/// Array's shape type whose number of dimensions and the size of each dimension
/// are known at compile time.
pub struct ShapeDyn;

/// Recursive macro generating type signature for `ConstShape` with a known
/// number of dimensions. `$n` is the number of elements of the current
/// dimension.
macro generate_const_shape_type {
    ($n:ident) => {
        ShapeConst<(), $n>
    },
    ($n:ident, $($ns:ident),+) => {
        ShapeConst<generate_const_shape_type!($($ns),+), $n>
    }
}

/// Macro counting the number of elements in a list of arguments.
macro count {
    ($x:tt) => { 1usize },
    ($x:tt, $($xs:tt),+) => { 1usize + count!($($xs),+) }
}

/// Macro calculating the product of the elements in a list of arguments.
macro product {
    ($x:tt) => { $x },
    ($x:tt, $($xs:tt),+) => { $x * product!($($xs),+) }
}

/// Macro implementing `Shape` and `FixedShape` for a fixed-sized
/// multi-dimensional array.
macro impl_const_shape {
    ($($n:ident),+) => {
        impl<$(const $n: usize),+> Shape for generate_const_shape_type!($($n),+) {
            type Type = [usize; count!($($n),+)];

            fn shape() -> Self::Type {
                [$($n),+]
            }
        }

        impl<$(const $n: usize),+> FixedShape for generate_const_shape_type!($($n),+) {
            const N_DIMS: usize = count!($($n),+);
            const N_ELEMS: usize = product!($($n),+);
        }
    },
}

impl_const_shape!(N0);
impl_const_shape!(N0, N1);
impl_const_shape!(N0, N1, N2);
impl_const_shape!(N0, N1, N2, N3);
impl_const_shape!(N0, N1, N2, N3, N4);
impl_const_shape!(N0, N1, N2, N3, N4, N5);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14);
impl_const_shape!(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15);

impl Shape for ShapeDyn {
    type Type = Vec<usize>;

    fn shape() -> Self::Type {
        Vec::with_capacity(16)
    }
}

/// Macro facilitating the creation of a `ShapeConst`.
pub macro cs {
($n:expr) => {
        ShapeConst<(), $n>
    },
($n:expr, $($tail:expr),*) => {
        ShapeConst<cs!($($tail),*), $n>
    }
}