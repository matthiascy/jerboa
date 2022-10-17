use linalg::MachineEpsilon;

/// Computes the magnitude of the conservative bounding of the relative error.
/// (1 \pm \epsilon_m) ^ n.
///
/// (1 \pm \epsilon_m) ^ n is bounded by 1 + \theta_n, where
/// |\theta_n| \leq \frac{n\epsilon_m}{1 - n\epsilon_m}
///
/// More details in  Higham, N. J. Accuracy and Stability of Numerical
/// Algorithms (2nd ed.). Philadelphia: Society for Industrial and Applied
/// Mathematics, (2002).
#[inline]
pub const fn mre_f32(n: u32) -> f32 {
    let t = f32::MACH_EPS * n as f32;
    t / (1.0 - t)
}

#[inline]
pub const fn mre_f64(n: u32) -> f64 {
    let t = f64::MACH_EPS * n as f64;
    t / (1.0 - t)
}
