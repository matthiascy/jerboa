use crate::core::Layout;
use core::marker::PhantomData;

/// Dimension sequence.
///
/// Trait for types that can be used as shape for an array.
pub trait DimSeq: Clone {
    fn as_slice(&self) -> &[usize];
    fn as_slice_mut(&mut self) -> &mut [usize];
}

macro impl_shape_storage($($n:expr),+) {
    $(
        impl DimSeq for [usize; $n] {
            fn as_slice(&self) -> &[usize] {
                self
            }

            fn as_slice_mut(&mut self) -> &mut [usize] {
                self
            }
        }
    )+
}

impl_shape_storage!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

impl DimSeq for Vec<usize> {
    fn as_slice(&self) -> &[usize] {
        self
    }

    fn as_slice_mut(&mut self) -> &mut [usize] {
        self
    }
}

/// Trait for fixed-sized multi-dimensional array with a known number of
/// dimensions and size of each dimension at compile time.
///
/// This trait is a helper to construct a concrete array shape from recursively
/// defined const shape type. See `ConstShape` for more details.
#[const_trait]
pub trait CShape {
    /// Underlying storage type for the shape.
    type UnderlyingType: DimSeq;

    /// Number of dimensions in the multi-dimensional array.
    const N_DIMS: usize;

    /// Number of elements in the multi-dimensional array.
    const N_ELEMS: usize;

    /// Shape of the multi-dimensional array. For fixed-sized shape, which is
    /// known at compile time, this is an array with known size at compile
    /// time.
    const SHAPE: Self::UnderlyingType;

    /// Strides of the multi-dimensional array: the number of elements needed to
    /// skip to get to the next element along each dimension. Pre-computed
    /// strides for row-major layout.
    const ROW_MAJOR_STRIDES: Self::UnderlyingType;

    /// Pre-computed strides for column-major layout.
    const COLUMN_MAJOR_STRIDES: Self::UnderlyingType;
}

impl const CShape for () {
    type UnderlyingType = [usize; 0];
    const N_DIMS: usize = 0;
    const N_ELEMS: usize = 0;
    const SHAPE: Self::UnderlyingType = [];
    const ROW_MAJOR_STRIDES: Self::UnderlyingType = [];
    const COLUMN_MAJOR_STRIDES: Self::UnderlyingType = [];
}

/// Array's const shape type.
///
/// This shape type embeds the number of dimensions and the size of each
/// dimension at compile time in the type system using recursive definition.
/// This allows the creation of multi-dimensional array on stack with known
/// number of dimensions and known size for each dimension.
///
/// This is a workaround for the lack of variadic generics in Rust. Once
/// variadic generics are available, this type will be removed.
///
/// To create a const shape, use the `s!` macro.
pub struct ConstShape<A: CShape, const N: usize>(PhantomData<[A; N]>);

/// Recursive macro generating type signature for `ConstShape` from a
/// sequence of integers representing the size of each dimension.
macro generate_const_shape {
    ($n:ident) => {
        ConstShape<(), $n>
    },
    ($n:ident, $($ns:ident),+) => {
        ConstShape<generate_const_shape!($($ns),+), $n>
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

/// Calculates the strides for row-major layout from the shape.
const fn calc_row_major_strides<const N: usize>(shape: &[usize; N]) -> [usize; N] {
    let mut strides = [1; N];
    let mut stride = 1;
    let mut i = N;
    while i > 0 {
        strides[i - 1] = stride;
        stride *= shape[i - 1];
        i -= 1;
    }
    strides
}

/// Calculates the strides for column-major layout from the shape.
const fn calc_col_major_strides<const N: usize>(shape: &[usize; N]) -> [usize; N] {
    let mut strides = [1; N];
    let mut stride = 1;
    let mut i = 0;
    while i < N {
        strides[i] = stride;
        stride *= shape[i];
        i += 1;
    }
    strides
}

/// Macro implementing `CShape` for `ConstShape` of a given shape.
macro impl_const_shape($($n:ident),+) {
    impl<$(const $n: usize),+> const CShape for generate_const_shape!($($n),+) {
        type UnderlyingType = [usize; count!($($n),+)];
        const N_DIMS: usize = count!($($n),+);
        const N_ELEMS: usize = product!($($n),+);
        const SHAPE: Self::UnderlyingType = [$($n),+];
        const ROW_MAJOR_STRIDES: Self::UnderlyingType = calc_row_major_strides(&[$($n),+]);
        const COLUMN_MAJOR_STRIDES: Self::UnderlyingType = calc_col_major_strides(&[$($n),+]);
    }
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

/// Macro facilitating the creation of a `ConstShape`.
pub macro s {
    ($n:expr) => {
        ConstShape<(), $n>
    },
    ($n:expr, $($tail:expr),*) => {
        ConstShape<s!($($tail),*), $n>
    }
}

/// Array's shape type whose number of dimensions and the size of each dimension
/// are unknown at compile time.
pub struct DynamicShape;

/// Common trait for array's shape type.
#[const_trait]
pub trait Shape {
    type UnderlyingType: DimSeq;

    /// Number of dimensions.
    const N_DIMS: Option<usize>;

    /// Number of elements.
    const N_ELEMS: Option<usize>;

    /// Shape of the array.
    const SHAPE: Option<Self::UnderlyingType>;

    /// Strides for row-major layout.
    const ROW_MAJOR_STRIDES: Option<Self::UnderlyingType>;

    /// Strides for column-major layout.
    const COLUMN_MAJOR_STRIDES: Option<Self::UnderlyingType>;

    /// Returns the concrete shape type of the array.
    ///
    /// In the case of const shape, this is the actual shape. On the other hand,
    /// in the case of dynamic shape, this is an instance of
    /// `Self::UnderlyingType`.
    fn shape() -> Self::UnderlyingType;

    /// Returns the concrete strides type of the array.
    ///
    /// In the case of const shape, the returned value is the actual strides
    /// of the array, cause the strides are known at compile time. Otherwise,
    /// the returned value is an empty value of type `Self::UnderlyingType`.
    fn row_major_strides() -> Self::UnderlyingType;

    /// Returns the concrete strides type of the array.
    fn column_major_strides() -> Self::UnderlyingType;
}

impl<T: CShape> const Shape for T {
    type UnderlyingType = T::UnderlyingType;
    const N_DIMS: Option<usize> = Some(T::N_DIMS);
    const N_ELEMS: Option<usize> = Some(T::N_ELEMS);
    const SHAPE: Option<Self::UnderlyingType> = Some(T::SHAPE);
    const ROW_MAJOR_STRIDES: Option<Self::UnderlyingType> = Some(T::ROW_MAJOR_STRIDES);
    const COLUMN_MAJOR_STRIDES: Option<Self::UnderlyingType> = Some(T::COLUMN_MAJOR_STRIDES);

    fn shape() -> Self::UnderlyingType {
        T::SHAPE
    }

    fn row_major_strides() -> Self::UnderlyingType {
        T::ROW_MAJOR_STRIDES
    }

    fn column_major_strides() -> Self::UnderlyingType {
        T::COLUMN_MAJOR_STRIDES
    }
}

impl Shape for DynamicShape {
    type UnderlyingType = Vec<usize>;
    const N_DIMS: Option<usize> = None;
    const N_ELEMS: Option<usize> = None;
    const SHAPE: Option<Self::UnderlyingType> = None;
    const ROW_MAJOR_STRIDES: Option<Self::UnderlyingType> = None;
    const COLUMN_MAJOR_STRIDES: Option<Self::UnderlyingType> = None;

    fn shape() -> Self::UnderlyingType {
        Vec::with_capacity(8)
    }

    fn row_major_strides() -> Self::UnderlyingType {
        Vec::with_capacity(8)
    }

    fn column_major_strides() -> Self::UnderlyingType {
        Vec::with_capacity(8)
    }
}

/// Computes the number of elements in the array given its shape.
pub const fn compute_num_elems(shape: &[usize]) -> usize {
    let mut n_elems = 1;
    let n = shape.len();
    let mut i = 0;
    while i < n {
        n_elems *= shape[i];
        i += 1;
    }
    n_elems
}

/// Computes the strides of the array given its shape and layout.
pub const fn compute_strides(shape: &[usize], strides: &mut [usize], layout: Layout) {
    let n = shape.len();
    let mut i = 0;
    match layout {
        Layout::RowMajor => {
            strides[n - 1] = 1;
            while i < n - 1 {
                strides[n - 2 - i] = strides[n - 1 - i] * shape[n - 1 - i];
                i += 1;
            }
        }
        Layout::ColumnMajor => {
            strides[0] = 1;
            while i < n - 1 {
                strides[i + 1] = strides[i] * shape[i];
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_macro() {
        type S3 = s![2, 3, 4];
        assert_eq!(<S3 as CShape>::N_DIMS, 3);
        assert_eq!(<S3 as Shape>::N_ELEMS, Some(24));
        assert_eq!(<S3 as CShape>::SHAPE, [2, 3, 4]);
        assert_eq!(<S3 as Shape>::row_major_strides(), [12, 4, 1]);
        assert_eq!(<S3 as Shape>::column_major_strides(), [1, 2, 6]);
    }

    #[test]
    fn test_calc_strides() {
        let mut strides = [0; 3];
        compute_strides(&[2, 3, 4], &mut strides, Layout::RowMajor);
        assert_eq!(strides, [12, 4, 1]);
        compute_strides(&[3, 4, 2], &mut strides, Layout::ColumnMajor);
        assert_eq!(strides, [1, 3, 12]);
    }

    #[test]
    fn test_n_elems() {
        assert_eq!(compute_num_elems(&[2, 3, 4]), 24);
        assert_eq!(compute_num_elems(&[3, 4, 2]), 24);
    }
}
