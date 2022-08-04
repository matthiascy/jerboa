use crate::core::axis::Axis;
use glam::{Vec2, Vec3};
use std::ops::{Index, IndexMut};

// todo: generic type

/// A 2D bounding box.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounds2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Default for Bounds2 {
    fn default() -> Self {
        Bounds2 {
            min: Vec2::new(f32::MAX, f32::MAX),
            max: Vec2::new(f32::MIN, f32::MIN),
        }
    }
}

impl Bounds2 {
    /// Creates a new bounding box with the given min and max.
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    // todo: const fn
    pub fn diagonal(&self) -> Vec2 {
        self.max - self.min
    }

    // todo: const fn
    pub fn area(&self) -> f32 {
        let diag = self.max - self.min;
        diag.x * diag.y
    }

    pub const fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    pub const fn is_degenerate(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    // todo: const fn
    pub fn max_axis(&self) -> Axis {
        let diag = self.diagonal();
        if diag.x > diag.y {
            Axis::X
        } else {
            Axis::Y
        }
    }
}

impl Index<usize> for Bounds2 {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Bounds2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.min,
            1 => &mut self.max,
            _ => panic!("index out of bounds"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounds3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for Bounds3 {
    fn default() -> Self {
        Bounds3 {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }
}

impl Bounds3 {
    /// Creates a new bounding box with the given min and max.
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub const fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    pub const fn is_degenerate(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    // todo: const fn
    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    // todo: const fn
    pub fn area(&self) -> f32 {
        let diag = self.max - self.min;
        2.0 * (diag.x * diag.y + diag.x * diag.z + diag.y * diag.z)
    }

    // todo: const fn
    pub fn volume(&self) -> f32 {
        let diag = self.max - self.min;
        diag.x * diag.y * diag.z
    }

    // todo: const fn
    pub fn max_axis(&self) -> Axis {
        let diag = self.diagonal();
        if diag.x > diag.y && diag.x > diag.z {
            Axis::X
        } else if diag.y > diag.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}
