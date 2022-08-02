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

pub trait Floating: MachineEpsilon {}

impl f32 for Floating {}
impl f64 for Floating {}

/// Returns the next representable floating point number.
///
/// Given a floating point number `val`, this function
/// increases the significand by one, where if the result
/// overflows, the significand is reset to zero and the
/// exponent is increased by one.
pub const fn next_float_up(val: f32) -> f32 {
    if val.is_infinite() && val.is_sign_positive() {
        return val;
    }

    let f = if val == -0.0 { 0.0 } else { val };

    let bits = f.to_bits();
    if f >= 0.0 {
        f32::from_bits(bits + 1)
    } else {
        f32::from_bits(bits - 1)
    }
}

/// Computes the magnitude of the conservative bounding of the relative error.
/// (1 \pm \epsilon_m) ^ n.
///
/// (1 \pm \epsilon_m) ^ n is bounded by 1 + \theta_n, where
/// |\theta_n| \leq \frac{n\epsilon_m}{1 - n\epsilon_m}
///
/// More details in  Higham, N. J. Accuracy and Stability of Numerical
/// Algorithms (2nd ed.). Philadelphia: Society for Industrial and Applied
/// Mathematics, (2002).
pub const fn mre<F: Floating>(n: u32) -> F {
    let t = n as f32 * F::MACH_EPS;
    t / (1.0 - t)
}

#[cfg(test)]
mod tests {
    use super::next_float_up;
    use quickcheck::quickcheck;

    quickcheck! {
        fn next_float(val: u32) -> bool {
            let val_u32 = val % (u32::MAX - 1);
            let next_u32 = val + 1;
            let next_f32 = next_float_up(val_u32 as f32);
            next_f32.to_bits() == next_u32
        }
    }
}
