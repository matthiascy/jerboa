use crate::rtc::color::RgbColor;
use crate::rtc::integrator::Integrator;
use crate::rtc::ray::Ray;
use crate::rtc::scene::Scene;

#[derive(Debug)]
pub struct RayCastIntegrator {}

impl Integrator for RayCastIntegrator {
    fn trace_ray(&self, scene: &Scene, ray: &Ray) -> RgbColor {
        todo!()
    }

    fn trace_ray_with_depth(&self, scene: &Scene, ray: &Ray, depth: u32) -> RgbColor {
        todo!()
    }
}