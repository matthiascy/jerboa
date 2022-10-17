use crate::rtc::{integrator::Integrator, shape::Sphere, view_plane::ViewPlane};
use glam::Vec3;

pub struct Scene {
    pub view_plane: ViewPlane,
    pub sphere: Sphere,
    pub integrator: Box<dyn Integrator>,
}

impl Scene {
    pub fn new(integrator: Box<dyn Integrator>) -> Self {
        Scene {
            view_plane: ViewPlane::new(512, 512, 1.0),
            sphere: Sphere::new(Vec3::ZERO, 1.0),
            integrator,
        }
    }
}
