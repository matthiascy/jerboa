mod integrators;
mod ray;
mod shapes;

use crate::rtc::ray::Ray;
use glam::Vec3;

pub struct IntersectRecord {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Shape {
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord>;
}
