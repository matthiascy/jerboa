use glam::Vec3;

pub struct Ray {
    /// The origin of the ray.
    pub o: Vec3,

    /// The direction of the ray.
    pub d: Vec3,

    /// Component wise reciprocal of the direction vector.
    pub d_rcp: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray {
            o,
            d,
            d_rcp: d.recip(),
        }
    }
}
