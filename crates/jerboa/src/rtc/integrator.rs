use crate::rtc::color::RgbColor;
use crate::rtc::ray::Ray;
use crate::rtc::scene::Scene;

mod ray_cast;

pub trait Integrator {
    fn trace_ray(&self, scene: &Scene, ray: &Ray) -> RgbColor;
    fn trace_ray_with_depth(&self, scene: &Scene, ray: &Ray, depth: u32) -> RgbColor;
}
