use std::ops::{Add, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct RgbColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RgbColor {
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0 };

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        RgbColor { r, g, b }
    }

    pub fn pow(&self, exp: f32) -> Self {
        RgbColor {
            r: self.r.powf(exp),
            g: self.g.powf(exp),
            b: self.b.powf(exp),
        }
    }
}

impl Add<RgbColor> for RgbColor {
    type Output = RgbColor;

    fn add(self, rhs: RgbColor) -> Self::Output {
        RgbColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for RgbColor {
    type Output = RgbColor;

    fn mul(self, rhs: f32) -> Self::Output {
        RgbColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<RgbColor> for f32 {
    type Output = RgbColor;

    fn mul(self, rhs: RgbColor) -> Self::Output {
        RgbColor {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<f32> for RgbColor {
    type Output = RgbColor;

    fn div(self, rhs: f32) -> Self::Output {
        RgbColor {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

// Component-wise multiplication for color mixing.
impl Mul for RgbColor {
    type Output = RgbColor;

    fn mul(self, rhs: Self) -> Self::Output {
        RgbColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
