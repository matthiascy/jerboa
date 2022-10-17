pub mod camera;
pub mod color;
mod integrator;
mod ray;
pub mod scene;
pub mod shape;
pub mod view_plane;

use crate::rtc::{ray::Ray, shape::Shape};
use glam::Vec3;

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
