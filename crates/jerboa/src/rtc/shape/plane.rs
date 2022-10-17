use crate::{
    core::rounding::mre_f32,
    rtc::{ray::Ray, IntersectRecord, Shape},
};
use glam::Vec3;

pub struct Plane {
    /// A point on the plane (in world frame).
    pub p: Vec3,

    /// The normal of the plane.
    pub n: Vec3,
}

impl Plane {
    pub fn new(p: Vec3, n: Vec3) -> Self {
        Self {
            p,
            n: n.normalize(),
        }
    }
}

// todo: floating point error
impl Shape for Plane {
    fn intersect_p(&self, ray: &Ray) -> bool {
        let denom = self.n.dot(ray.d);
        if denom >= denom * mre_f32(1) {
            // Ray is on the wrong side of the plane.
            false
        } else {
            let t = self.n.dot(self.p - ray.o) / denom;
            t >= t * mre_f32(4)
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord> {
        let denom = self.n.dot(ray.d);
        if denom >= denom * mre_f32(2) {
            None
        } else {
            let t = self.n.dot(self.p - ray.o) / denom;
            let mre = mre_f32(4);
            if t < t * mre {
                return None;
            }
            let p = ray.o + ray.d * t;
            Some(IntersectRecord { t, p, n: self.n })
        }
    }
}
