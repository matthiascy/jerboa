mod integrator;
mod ray;
pub mod shape;
pub mod scene;
pub mod color;
pub mod view_plane;
pub mod camera;

use crate::rtc::ray::Ray;
use glam::Vec3;
use crate::rtc::shape::Shape;

#[derive(Debug)]
pub struct IntersectRecord {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Material {}

pub struct Primitive {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
    pub transform: glam::Mat4,
}

pub struct SceneGraph {
    pub primitives: Vec<Primitive>,
}
