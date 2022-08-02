use crate::rtc::{IntersectRecord, ray::Ray, Shape};
use glam::Vec3;

pub struct Plane {
    /// A point on the plane.
    pub p: Vec3,

    /// The normal of the plane.
    pub n: Vec3,
}

// todo: floating point error analysis
impl Shape for Plane {
    fn intersect_p(&self, ray: &Ray) -> bool {
        if denom >= 0.0 {
            // Ray is on the wrong side of the plane.
            false
        } else {
            let t = self.normal.dot(self.p - ray.o) / denom;
            t >= 0.0
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord> {
        let denom = self.normal.dot(ray.d);
        if denom > 0.0 {
            return None;
        }
        let t = self.point.dot(self.normal) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.o + ray.d * t;
        let n = self.normal;
        Some(IntersectRecord { t, p, n })
    }
}
