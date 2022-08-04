use crate::core::Angle;
use glam::Vec3;
use crate::rtc::IntersectRecord;
use crate::rtc::ray::Ray;
use crate::rtc::shape::Shape;

#[derive(Debug)]
pub struct Sphere {
    /// The center of the sphere (in world frame).
    pub c: Vec3,

    /// The radius of the sphere.
    pub r: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Sphere { c, r }
    }
}

// todo: floating point error
impl Shape for Sphere {
    fn intersect_p(&self, ray: &Ray) -> bool {
        let oc = ray.o - self.c;
        let a = ray.d.dot(ray.d);
        let b = oc.dot(ray.d) * 2.0;
        let c = oc.dot(oc) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord> {
        let oc = ray.o - self.c;
        let a = ray.d.dot(ray.d);
        let b = oc.dot(ray.d) * 2.0;
        let c = oc.dot(oc) - self.r * self.r;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            None
        } else {
            let e = disc.sqrt();
            let denom = 2.0 * a;
            let t1 = (-b - e) / denom;
            let t2 = (-b + e) / denom;
            let t = if t1 < t2 { t1 } else { t2 };
            if t >= 0.0 {
                let p = ray.o + ray.d * t;
                let n = (p - self.c).normalize();
                Some(IntersectRecord {
                    t,
                    p,
                    n,
                })
            } else {
                None
            }
        }
    }
}
