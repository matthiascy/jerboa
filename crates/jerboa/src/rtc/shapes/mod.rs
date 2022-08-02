mod plane;

pub use plane::Plane;

use glam::Vec3;

pub struct Sphere {
    /// The center of the sphere.
    pub c: Vec3,

    /// The radius of the sphere.
    pub r: f32,
}
