use crate::{packet::Packet, traits::Num, Normal, Point, Vector};
use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Column-major matrix.
#[repr(C)]
pub struct Matrix<T: Num, const R: usize, const C: usize>(Packet<Packet<T, R>, C>);

impl<T: Num, const R: usize, const C: usize> Clone for Matrix<T, R, C> {
    fn clone(&self) -> Self {
        Matrix(self.0)
    }
}

impl<T: Num, const R: usize, const C: usize> Copy for Matrix<T, R, C> {}

impl<T: Num, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Num + Eq, const R: usize, const C: usize> Eq for Matrix<T, R, C> {}

impl<T: Num, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = Packet<T, R>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const R: usize, const C: usize> Debug for Matrix<T, R, C>
where
    T: Num + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for col in 0..C {
            if col != C - 1 {
                writeln!(f, "{:?},", self.0[col])?;
            } else {
                write!(f, "{:?}", self.0[col])?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T, const R: usize, const C: usize> Display for Matrix<T, R, C>
where
    T: Num + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for col in 0..C {
            if col != C - 1 {
                writeln!(f, "{},", self.0[col])?;
            } else {
                write!(f, "{}", self.0[col])?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Num + Default,
{
    fn default() -> Self {
        Matrix(unsafe { core::mem::zeroed::<Packet<Packet<T, R>, C>>() })
    }
}

impl<T: Num, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn shape(&self) -> (usize, usize) {
        (R, C)
    }

    pub const fn is_square(&self) -> bool {
        R == C
    }

    /// Constructs a matrix from a slice of columns.
    pub fn from_cols(cols: [[T; R]; C]) -> Self {
        Matrix(Packet::new(cols.map(|col| Packet::new(col))))
    }

    /// Constructs a matrix from a slice of rows.
    pub fn from_rows(rows: [[T; C]; R]) -> Self {
        let mut cols = [[T::zero(); R]; C];
        for (i, row) in rows.into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                cols[j][i] = col;
            }
        }
        Matrix::from_cols(cols)
    }

    /// Returns the column vector at the given index.
    pub fn col(&self, c: usize) -> Vector<T, R> {
        Vector(self.0[c])
    }

    /// Returns the row vector at the given index.
    pub fn row(&self, r: usize) -> Vector<T, C> {
        let mut row = unsafe { ::core::mem::zeroed::<[T; C]>() };
        for (n, val) in row.iter_mut().enumerate() {
            *val = self.0[n][r];
        }
        Vector::new(row)
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut data = [[T::zero(); C]; R];
        for r in 0..R {
            for c in 0..C {
                data[c][r] = self.0[r][c];
            }
        }
        Matrix::from_cols(data)
    }
}

// Implementations for operators between two matrices.
macro_rules! impl_matrix_binary_op {
    ($mat_a:ident $({$op_trait:ident, $op:ident})|* $mat_b:ident = $mat_c:ident) => {
        $(
            impl<T: Num, const R: usize, const C: usize> $op_trait<$mat_b<T, R, C>> for $mat_a<T, R, C>{
                type Output = $mat_c<T, R, C>;
                fn $op(self, rhs: Self) -> Self::Output {
                    Matrix(self.0.$op(rhs.0))
                }
            }

            impl<'a, T: Num, const R: usize, const C: usize> $op_trait<&'a $mat_b<T, R, C>> for $mat_a<T, R, C> {
                type Output = $mat_c<T, R, C>;
                fn $op(self, rhs: &'a $mat_b<T, R, C>) -> Self::Output {
                    Matrix(self.0.$op(rhs.0))
                }
            }

            impl<'a, T: Num, const R: usize, const C: usize> $op_trait<$mat_b<T, R, C>> for &'a $mat_a<T, R, C> {
                type Output = $mat_c<T, R, C>;
                fn $op(self, rhs: $mat_b<T, R, C>) -> Self::Output {
                    Matrix(self.0.$op(rhs.0))
                }
            }

            impl<'a, 'b, T: Num, const R: usize, const C: usize> $op_trait<&'a $mat_b<T, R, C>> for &'b $mat_a<T, R, C> {
                type Output = $mat_c<T, R, C>;
                fn $op(self, rhs: &'a $mat_b<T, R, C>) -> Self::Output {
                    Matrix(self.0.$op(rhs.0))
                }
            }
        )*
    };
    (asgmt $mat_a:ident $({$op_trait:ident, $op:ident})|* $mat_b:ident = $mat_c:ident) => {
        $(
            impl<T: Num, const R: usize, const C: usize> $op_trait<$mat_b<T, R, C>> for $mat_a<T, R, C> {
                fn $op(&mut self, rhs: Self) {
                    self.0.$op(rhs.0);
                }
            }

            impl<'a, T: Num, const R: usize, const C: usize> $op_trait<&'a $mat_b<T, R, C>> for $mat_a<T, R, C> {
                fn $op(&mut self, rhs: &'a $mat_b<T, R, C>) {
                    self.0.$op(rhs.0);
                }
            }
        )*
    };
}

// The shared part of the implementation of matrix product.
macro_rules! matrix_product_body {
    ($mat:ident, $self:ident, $rhs:ident) => {{
        let mut cols: [[T; R]; C] = unsafe { ::core::mem::zeroed() };
        for c in 0..C {
            for r in 0..R {
                cols[c][r] = $self.row(r).0.dot(&$rhs.col(c).0);
            }
        }
        $mat::from_cols(cols)
    }};
}

// The implementation of matrix product for matrices.
macro_rules! impl_matrix_product {
    ($mat:ident) => {
        impl<T: Num, const R: usize, const C: usize, const K: usize> Mul<$mat<T, K, C>>
            for $mat<T, R, K>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $mat<T, R, C>;
            fn mul(self, rhs: $mat<T, K, C>) -> Self::Output {
                matrix_product_body!($mat, self, rhs)
            }
        }

        impl<'a, T: Num, const R: usize, const C: usize, const K: usize> Mul<&'a $mat<T, K, C>>
            for $mat<T, R, K>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $mat<T, R, C>;
            fn mul(self, rhs: &'a $mat<T, K, C>) -> Self::Output {
                matrix_product_body!($mat, self, rhs)
            }
        }

        impl<'a, T: Num, const R: usize, const C: usize, const K: usize> Mul<$mat<T, K, C>>
            for &'a $mat<T, R, K>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $mat<T, R, C>;
            fn mul(self, rhs: $mat<T, K, C>) -> Self::Output {
                matrix_product_body!($mat, self, rhs)
            }
        }

        impl<'a, 'b, T: Num, const R: usize, const C: usize, const K: usize> Mul<&'a $mat<T, K, C>>
            for &'b $mat<T, R, K>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $mat<T, R, C>;
            fn mul(self, rhs: &'a $mat<T, K, C>) -> Self::Output {
                matrix_product_body!($mat, self, rhs)
            }
        }
    };
}

// The shared part of the implementation of matrix vector product.
macro_rules! matrix_vector_product_body {
    ($mat:ident, $vec:ident, $self:ident, $rhs:ident) => {{
        let mut data: [T; C] = unsafe { ::core::mem::zeroed() };
        for r in 0..C {
            data[r] = $self.row(r).0.dot(&$rhs.0);
        }
        $vec::new(data)
    }};
}

// The implementation of matrix vector product.
macro_rules! impl_matrix_vector_product {
    ($mat:ident, $vec:ident) => {
        impl<T: Num, const R: usize, const C: usize> Mul<$vec<T, C>> for $mat<T, R, C>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $vec<T, C>;

            fn mul(self, rhs: $vec<T, C>) -> Self::Output {
                matrix_vector_product_body!($mat, $vec, self, rhs)
            }
        }

        impl<'a, T: Num, const R: usize, const C: usize> Mul<$vec<T, C>> for &'a $mat<T, R, C>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $vec<T, C>;

            fn mul(self, rhs: $vec<T, C>) -> Self::Output {
                matrix_vector_product_body!($mat, $vec, self, rhs)
            }
        }

        impl<'a, T: Num, const R: usize, const C: usize> Mul<&'a $vec<T, C>> for $mat<T, R, C>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $vec<T, C>;

            fn mul(self, rhs: &'a $vec<T, C>) -> Self::Output {
                matrix_vector_product_body!($mat, $vec, self, rhs)
            }
        }

        impl<'a, 'b, T: Num, const R: usize, const C: usize> Mul<&'b $vec<T, C>>
            for &'a $mat<T, R, C>
        where
            for<'e> &'e T: Mul<&'e T, Output = T>,
        {
            type Output = $vec<T, C>;

            fn mul(self, rhs: &'b $vec<T, C>) -> Self::Output {
                matrix_vector_product_body!($mat, $vec, self, rhs)
            }
        }
    };
}

// The implementation of scalar matrix product.
macro_rules! impl_matrix_scalar_binary_op {
    ($mat:ident $({$op_trait:ident, $op:ident})|*) => {
        $(
            impl<T: Num, const R: usize, const C: usize> $op_trait<T> for $mat<T, R, C> {
                type Output = $mat<T, R, C>;
                fn $op(self, rhs: T) -> Self {
                    $mat(self.0.map(|x| x.$op(rhs)))
                }
            }

            impl<'a, T: Num, const R: usize, const C: usize> $op_trait<T> for &'a $mat<T, R, C> {
                type Output = Matrix<T, R, C>;
                fn $op(self, rhs: T) -> Self::Output {
                    $mat(self.0.map(|x| x.$op(rhs)))
                }
            }
        )*
    };
    (asgmt $mat:ident $({$op_trait:ident, $op:ident})|*) => {
        $(
            impl<T: Num, const R: usize, const C: usize> $op_trait<T> for $mat<T, R, C> {
                fn $op(&mut self, rhs: T) {
                    for i in 0..C {
                        self.0[i].$op(rhs);
                    }
                }
            }
        )*
    };
}

macro_rules! impl_scalar_matrix_binary_op_ {
    ($({$op_trait:ident, $op:ident})|* $t:ty) => {
        $(
            impl<const R: usize, const C: usize> $op_trait<Matrix<$t, R, C>> for $t {
                type Output = Matrix<$t, R, C>;
                fn $op(self, rhs: Matrix<$t, R, C>) -> Self::Output {
                    rhs.$op(self)
                }
            }
        )*
    };
}

macro_rules! impl_scalar_matrix_binary_op {
    ($($t:ty)*) => {
        $(
            impl_scalar_matrix_binary_op_!({Mul, mul}|{Div, div} $t);
        )*
    };
}

impl_matrix_product!(Matrix);
impl_matrix_binary_op!(Matrix {Add, add} | {Sub, sub} | {Div, div} Matrix = Matrix);
impl_matrix_binary_op!(asgmt Matrix {AddAssign, add_assign} | {SubAssign, sub_assign} | {DivAssign, div_assign} Matrix = Matrix);
impl_matrix_scalar_binary_op!(Matrix {Mul, mul} | {Div, div});
impl_matrix_scalar_binary_op!(asgmt Matrix {MulAssign, mul_assign} | {DivAssign, div_assign});
impl_scalar_matrix_binary_op!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64);
impl_matrix_vector_product!(Matrix, Vector);
impl_matrix_vector_product!(Matrix, Normal);
impl_matrix_vector_product!(Matrix, Point);

impl<T, const R: usize, const C: usize> Neg for Matrix<T, R, C>
where
    T: Num + Neg<Output = T>,
{
    type Output = Matrix<T, R, C>;
    fn neg(self) -> Self::Output {
        Matrix(self.0.map(|x| -x))
    }
}

pub fn mat2<T: Num>(cols: [[T; 2]; 2]) -> Matrix<T, 2, 2> {
    Matrix::from_cols(cols)
}

pub fn mat3<T: Num>(cols: [[T; 3]; 3]) -> Matrix<T, 3, 3> {
    Matrix::from_cols(cols)
}

pub fn mat4<T: Num>(cols: [[T; 4]; 4]) -> Matrix<T, 4, 4> {
    Matrix::from_cols(cols)
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Normal, Point, Vector};

    #[test]
    fn ctor_from_rows() {
        let m = Matrix::from_rows([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m, Matrix::from_cols([[1, 4, 7], [2, 5, 8], [3, 6, 9]]));
    }

    #[test]
    fn debug_display() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        assert_eq!(format!("{:?}", a), "[[1, 2],\n[3, 4]]");

        let b = Matrix::from_cols([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(format!("{}", b), "[[1, 2, 3],\n[4, 5, 6]]");
    }

    #[test]
    fn add() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[5, 6], [7, 8]]);
        assert_eq!(a + b, Matrix::from_cols([[6, 8], [10, 12]]));
    }

    #[test]
    fn mul() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[5, 6], [7, 8]]);
        assert_eq!(a * b, Matrix::from_cols([[23, 34], [31, 46]]));
    }

    #[test]
    fn mul_scalar() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        assert_eq!(a * 2, Matrix::from_cols([[2, 4], [6, 8]]));

        let b = Matrix::from_cols([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(&b * 2, Matrix::from_cols([[2, 4, 6], [8, 10, 12]]));

        let c = Matrix::from_cols([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(2 * c, Matrix::from_cols([[2, 4, 6], [8, 10, 12]]));
    }

    #[test]
    fn mul_assign_scalar() {
        let mut a = Matrix::from_cols([[1, 2], [3, 4]]);
        a *= 2;
        assert_eq!(a, Matrix::from_cols([[2, 4], [6, 8]]));
    }

    #[test]
    fn add_assign() {
        let mut a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[1, 2], [3, 4]]);
        a += b;
        assert_eq!(a, Matrix::from_cols([[2, 4], [6, 8]]));
    }

    #[test]
    fn sub_assign() {
        let mut a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[1, 2], [3, 4]]);
        a -= b;
        assert_eq!(a, Matrix::from_cols([[0, 0], [0, 0]]));
    }

    #[test]
    fn div_assign() {
        let mut a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[1, 2], [3, 4]]);
        a /= b;
        assert_eq!(a, Matrix::from_cols([[1, 1], [1, 1]]));
    }

    #[test]
    fn row_access() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = a.row(0);
        let c = a.row(1);
        assert_eq!(b, Vector::new([1, 3]));
        assert_eq!(c, Vector::new([2, 4]));
    }

    #[test]
    fn col_access() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = a.col(0);
        let c = a.col(1);
        assert_eq!(b, Vector::new([1, 2]));
        assert_eq!(c, Vector::new([3, 4]));
    }

    #[test]
    fn product() {
        let a = Matrix::from_cols([[1, 2], [3, 4]]);
        let b = Matrix::from_cols([[5, 6], [7, 8], [9, 10]]);
        let c = a * b;
        assert_eq!(c, Matrix::from_cols([[23, 34], [31, 46], [39, 58]]));
    }

    #[test]
    fn vector_product() {
        let a = Matrix::from_cols([[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let v = Vector::new([1.0, 0.0, 0.0]);
        let r = a * v;
        assert_eq!(r, Vector::new([0.0, 1.0, 0.0]));
    }

    #[test]
    fn point_product() {
        let a = Matrix::from_cols([
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let p = Point::new([1.0, 0.0, 0.0, 1.0]);
        let r = a * p;
        assert_eq!(r, Point::new([0.0, 1.0, 0.0, 1.0]));
    }

    #[test]
    fn normal_product() {
        let a = Matrix::from_cols([[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let v = Normal::new([1.0, 0.0, 0.0]);
        let r = a * v;
        assert_eq!(r, Normal::new([0.0, 1.0, 0.0]));
    }
}
