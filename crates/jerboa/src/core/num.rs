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

impl Floating for f32 {}
impl Floating for f64 {}
