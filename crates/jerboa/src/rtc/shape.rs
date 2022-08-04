//! All shapes are defined in object coordinate space, which means that the
//! shape's origin is at (0, 0, 0).

mod disk;
mod plane;
mod sphere;

use crate::rtc::{ray::Ray, IntersectRecord};

pub trait Shape {
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord>;
}

pub use plane::Plane;
pub use sphere::Sphere;
