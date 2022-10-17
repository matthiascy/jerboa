use std::{
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Definition of additive identity.
/// ```text
/// x + 0 = x for all x ∈ 𝑭ⁿ
/// ```
pub trait Zero: Sized {
    /// Returns the additive identity.
    fn zero() -> Self;

    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool;
}

// TODO: const fn
/// Returns the additive identity.
#[inline(always)]
pub fn zero<T: Zero>() -> T {
    T::zero()
}

/// Definition of multiplicative identity.
/// ```text
/// x * 1 = x for all x ∈ 𝑭ⁿ
/// ```
pub trait One: Sized {
    /// Returns the multiplicative identity.
    fn one() -> Self;

    /// Returns `true` if `self` is equal to the multiplicative identity.
    fn is_one(&self) -> bool;
}

// TODO: const fn
/// Returns the multiplicative identity.
#[inline(always)]
pub fn one<T: One>() -> T {
    T::one()
}

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

/// Basic numeric class.
pub trait Num: Zero + One + NumOps + NumAssignOps + PartialEq + PartialOrd + Copy + Clone {
    fn negate(self) -> Self;
    fn signum(self) -> Self;
}

/// Machine epsilon for floating point types.
///
/// In rust/c++, this is the magnitude of one ulp (unit in the last place)
/// above the number 1. Because the absolute rounding error in default rounding
/// mode (round to nearest, ties to even) is no more than half of one ulp, the
/// machine epsilon is defined as half of the ulp.
pub trait MachineEpsilon {
    const MACH_EPS: Self;
}

impl MachineEpsilon for f32 {
    const MACH_EPS: Self = f32::EPSILON * 0.5;
}

impl MachineEpsilon for f64 {
    const MACH_EPS: Self = f64::EPSILON * 0.5;
}

pub trait Integral: Num {}

pub trait Floating: Num + MachineEpsilon // + Sqrt<Output = Self> + Abs<Output = Self>
{
    fn recip(self) -> Self;
    fn nan() -> Self;
    fn inf() -> Self;
    fn inf_neg() -> Self;
    fn pi() -> Self;
    fn epsilon() -> Self;
    fn mach_eps() -> Self;
    fn e() -> Self;
    fn frac_1_pi() -> Self;
    fn frac_1_sqrt_2() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrt_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;
    fn ln_2() -> Self;
    fn ln_10() -> Self;
    fn log2_10() -> Self;
    fn log2_e() -> Self;
    fn log10_2() -> Self;
    fn log10_e() -> Self;
    fn sqrt2() -> Self;
    fn tau() -> Self;

    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sub_normal(self) -> bool;
    fn is_negative_zero(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn classify(self) -> FpCategory;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    // TODO: fn sin_cos(self) -> (Self, Self);
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    fn hypot(self, other: Self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;

    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;

    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
}

macro_rules! impl_identity {
    ($($t:ty)*, $add_id:expr, $mul_id:expr) => {
        $(
            impl Zero for $t {
                #[inline]
                fn zero() -> Self {
                    $add_id
                }

                #[inline]
                fn is_zero(&self) -> bool {
                    *self == $add_id
                }
            }

            impl One for $t {
                #[inline]
                fn one() -> Self {
                    $mul_id
                }

                #[inline]
                fn is_one(&self) -> bool {
                    *self == $mul_id
                }
            }
        )*
    };
}

macro_rules! impl_num_ops {
    ($($t:ty)*) => {
        $(
            impl NumOps for $t {}
            impl NumAssignOps for $t {}
        )*
    };
}

macro_rules! impl_num_trait {
    (@signed {$($t:ty)*}, $add_id:expr, $mul_id:expr) => {
        impl_identity!($($t)*, $add_id, $mul_id);
        impl_num_ops!($($t)*);
        $(
            impl Num for $t {
                #[inline]
                fn negate(self) -> Self {
                    -self
                }

                #[inline]
                fn signum(self) -> Self {
                    if self < 0 as $t {
                        -1 as $t
                    } else if self > 0 as $t {
                        1 as $t
                    } else {
                        0 as $t
                    }
                }
            }
        )*
    };
    (@unsigned {$($t:ty)*}, $add_id:expr, $mul_id:expr) => {
        impl_identity!($($t)*, $add_id, $mul_id);
        impl_num_ops!($($t)*);
        $(
            impl Num for $t {
                #[inline]
                fn negate(self) -> Self {
                    panic!("Negate called on unsigned type!");
                }

                #[inline]
                fn signum(self) -> Self {
                    if self > 0 as $t {
                        1 as $t
                    } else {
                        0 as $t
                    }
                }
            }
        )*
    };
}

macro_rules! forward_floating_constant {
    ($($constant:ident () -> $ret:expr;)+) => {$(
        #[inline]
        fn $constant() -> Self {
            $ret
        }
    )+};
}

macro_rules! forward_floating_method {
    ($($method:ident ($($args:ident : $argt:ty),*) -> $t:ty;)+) => {
        $(
            fn $method(self, $($args: $argt),*) -> $t {
                self.$method($($args),*)
            }
        )+
    };
}

macro_rules! impl_floating {
    ($($t:ty, $p:path),*) => {$(
        impl Floating for $t {
            forward_floating_constant! {
                nan() -> <$t>::NAN;
                inf() -> <$t>::INFINITY;
                inf_neg() -> <$t>::NEG_INFINITY;
                epsilon() -> <$t>::EPSILON;
                pi() -> { use $p as base; base::PI };
                e() -> { use $p as base; base::E };
                frac_1_pi() -> { use $p as base; base::FRAC_1_PI };
                frac_1_sqrt_2() -> { use $p as base; base::FRAC_1_SQRT_2 };
                frac_2_pi() -> { use $p as base; base::FRAC_2_PI };
                frac_2_sqrt_pi() -> {use $p as base; base::FRAC_2_SQRT_PI };
                frac_pi_2() -> { use $p as base; base::FRAC_PI_2 };
                frac_pi_3() -> { use $p as base; base::FRAC_PI_3 };
                frac_pi_4() -> { use $p as base; base::FRAC_PI_4 };
                frac_pi_6() -> { use $p as base; base::FRAC_PI_6 };
                frac_pi_8() -> { use $p as base; base::FRAC_PI_8 };
                ln_2() -> { use $p as base; base::LN_2 };
                ln_10() -> { use $p as base; base::LN_10 };
                log2_10() -> { use $p as base; base::LOG2_10 };
                log2_e() -> { use $p as base; base::LOG2_E };
                log10_2() -> { use $p as base; base::LOG10_2 };
                log10_e() -> { use $p as base; base::LOG10_E };
                sqrt2() -> { use $p as base; base::SQRT_2 };
                tau() -> { use $p as base; base::TAU };
           }

            fn mach_eps() -> Self {
                <$t as MachineEpsilon>::MACH_EPS
            }

            forward_floating_method!{
                is_nan() -> bool;
                is_infinite() -> bool;
                is_finite() -> bool;
                is_normal() -> bool;
                is_sign_negative() -> bool;
                is_sign_positive() -> bool;
                floor() -> $t;
                ceil() -> $t;
                round() -> $t;
                trunc() -> $t;
                fract() -> $t;
                abs() -> $t;
                clamp(min: $t, max: $t) -> $t;
                classify() -> FpCategory;
                sin() -> $t;
                cos() -> $t;
                tan() -> $t;
                asin() -> $t;
                acos() -> $t;
                atan() -> $t;
                atan2(other: $t) -> $t;
                sinh() -> $t;
                cosh() -> $t;
                tanh() -> $t;
                asinh() -> $t;
                acosh() -> $t;
                atanh() -> $t;
                sqrt() -> $t;
                cbrt() -> $t;
                exp() -> $t;
                exp2() -> $t;
                ln() -> $t;
                log2() -> $t;
                log10() -> $t;
                exp_m1() -> $t;
                ln_1p() -> $t;
                recip() -> $t;
                hypot(other: $t) -> $t;
                mul_add(a: $t, b: $t) -> $t;
                div_euclid(rhs: $t) -> $t;
                rem_euclid(rhs: $t) -> $t;
                powi(n: i32) -> $t;
                powf(n: $t) -> $t;
                log(base: $t) -> $t;
            }

            fn is_sub_normal(self) -> bool {
                (0.0..=Self::MIN_POSITIVE).contains(&self)
            }

            fn is_negative_zero(self) -> bool {
                self == -0.0
            }
        }
    )*}
}

macro_rules! impl_integral {
    ($($t:ty)*) => {
        $(
            impl Integral for $t {}
        )*
    };
}

impl_num_trait!(@signed { i8 i16 i32 i64 isize }, 0, 1);
impl_num_trait!(@signed { f32 f64 }, 0.0, 1.0);
impl_num_trait!(@unsigned { u8 u16 u32 u64 usize }, 0, 1);
impl_floating!(f32, std::f32::consts, f64, std::f64::consts);
impl_integral!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
