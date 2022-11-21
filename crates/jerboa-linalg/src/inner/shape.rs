use std::marker::PhantomData;

pub trait ShapeStorage {
    fn as_slice(&self) -> &[usize];
}

macro impl_shape_storage($($n:expr),+) {
    $(
        impl ShapeStorage for [usize; $n] {
            fn as_slice(&self) -> &[usize] {
                self
            }
        }
    )+
}

impl_shape_storage!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

impl ShapeStorage for Vec<usize> {
    fn as_slice(&self) -> &[usize] {
        self
    }
}


/// Trait for array shape.
#[const_trait]
pub trait Shape {
    /// Underlying shape storage type.
    type UnderlyingType: ShapeStorage;

    /// The value of the underlying shape storage. For constant shape,
    /// this is an array with known size at compile time. For dynamic shape,
    /// this is a `Vec`.
    fn value() -> Self::UnderlyingType;

    /// The number of elements needed to skip to get to the next element along
    /// each dimension.
    fn strides() -> Self::UnderlyingType;
}

/// Trait for fixed-sized multi-dimensional array with a known number of
/// dimensions and a known size for each dimension.
#[const_trait]
pub trait FixedShape: Shape {
    /// Number of dimensions in the multi-dimensional array.
    const N_DIMS: usize;

    /// Number of elements in the multi-dimensional array.
    const N_ELEMS: usize;

    /// Shape of the multi-dimensional array.
    const SHAPE: Self::UnderlyingType;
}

impl const Shape for () {
    type UnderlyingType = [usize; 0];

    fn value() -> Self::UnderlyingType {
        []
    }

    fn strides() -> Self::UnderlyingType {
        []
    }
}

impl const FixedShape for () {
    const N_DIMS: usize = 0;
    const N_ELEMS: usize = 0;
    const SHAPE: Self::UnderlyingType = [];
}

/// Array's const shape type.
///
/// This shape type embeds the number of dimensions and the size of each dimension
/// at compile time in the type system using recursive definition. This allows
/// the creation of multi-dimensional array on stack with known number of dimensions
/// and known size for each dimension.
///
/// This is a workaround for the lack of variadic generics in Rust. Once variadic
/// generics are available, this type will be removed.
///
/// To create a const shape, use the `cs!` macro.
pub struct ShapeConst<A: FixedShape, const N: usize>(PhantomData<[A; N]>);

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
        impl<$(const $n: usize),+> const Shape for generate_const_shape_type!($($n),+) {
            type UnderlyingType = [usize; count!($($n),+)];

            fn value() -> Self::UnderlyingType {
                [$($n),+]
            }

            fn strides() -> Self::UnderlyingType {
                let shape = [$($n),+];
                const N: usize = count!($($n),+);
                let mut strides = [1; N];
                let mut i = 0;
                while i < N {
                    let mut j = i + 1;
                    while j < N {
                        strides[i] *= shape[j];
                        j += 1;
                    }
                    i += 1;
                }
                strides
            }
        }

        impl<$(const $n: usize),+> const FixedShape for generate_const_shape_type!($($n),+) {
            const N_DIMS: usize = count!($($n),+);
            const N_ELEMS: usize = product!($($n),+);
            const SHAPE: Self::UnderlyingType = [$($n),+];
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

/// Macro facilitating the creation of a `ShapeConst`.
pub macro cs {
    ($n:expr) => {
        ShapeConst<(), $n>
    },
    ($n:expr, $($tail:expr),*) => {
        ShapeConst<cs!($($tail),*), $n>
    }
}


/// Array's shape type whose number of dimensions and the size of each dimension
/// are unknown at compile time.
pub struct ShapeDyn;

impl Shape for ShapeDyn {
    type UnderlyingType = Vec<usize>;

    fn value() -> Self::UnderlyingType {
        Vec::with_capacity(8)
    }

    fn strides() -> Self::UnderlyingType {
        Vec::with_capacity(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let shape = <cs!(2, 3, 4) as Shape>::value();
        let strides = <cs!(2, 3, 4) as Shape>::strides();
        assert_eq!(shape, [2, 3, 4]);
        assert_eq!(strides, [12, 4, 1]);

        let shape1 = <cs!(3, 2, 4) as Shape>::value();
        let strides1 = <cs!(3, 2, 4) as Shape>::strides();
        assert_eq!(shape1, [3, 2, 4]);
        assert_eq!(strides1, [8, 4, 1]);
    }
}