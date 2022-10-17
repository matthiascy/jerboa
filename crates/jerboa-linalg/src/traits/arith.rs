// todo: const fn

/// Trait for square root.
pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

/// Trait for absolute value.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Trait for dot product.
pub trait DotProduct<Rhs> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// Trait for norm.
pub trait Norm {
    type Output;

    fn sqr_norm(self) -> Self::Output;
    fn norm(self) -> Self::Output;
}

/// Trait for signum.
pub trait Signum {
    type Output;

    fn signum(self) -> Self::Output;
}

/// Trait for reciprocal.
pub trait Recip {
    type Output;

    fn recip(self) -> Self::Output;
}

/// Trait for summation of elements.
pub trait Sum {
    type Output;

    fn sum(self) -> Self::Output;
}

/// Trait for normalization.
pub trait Normalization {
    type Output;

    fn normalize(self) -> Self::Output;
}

macro_rules! impl_sqrt {
    ($($t:ty)*) => {
        $(
            impl Sqrt for $t {
                type Output = Self;

                fn sqrt(self) -> Self::Output {
                    <$t>::sqrt(self)
                }
            }

            impl<'a> Sqrt for &'a $t {
                type Output = $t;

                fn sqrt(self) -> Self::Output {
                    <$t>::sqrt(*self)
                }
            }
        )*
    };
}

macro_rules! impl_abs {
    ($($t:ty)*) => {
        $(
            impl Abs for $t {
                type Output = $t;

                fn abs(self) -> Self::Output {
                    <$t>::abs(self)
                }
            }
        )*
    };
}

macro_rules! impl_signum {
    ($($t:ty)*) => {
        $(
            impl Signum for $t {
                type Output = $t;

                fn signum(self) -> Self {
                    <$t>::signum(self)
                }
            }
        )*
    };
}

macro_rules! impl_reciprocate {
    ($($t:ty)*) => {
        $(
            impl Recip for $t {
                type Output = $t;

                fn recip(self) -> Self {
                    <$t>::recip(self)
                }
            }
        )*
    };
}

impl_sqrt!(f32 f64);
impl_abs!(i8 i16 i32 i64 isize f32 f64);
impl_signum!(i8 i16 i32 i64 isize f32 f64);
impl_reciprocate!(f32 f64);
