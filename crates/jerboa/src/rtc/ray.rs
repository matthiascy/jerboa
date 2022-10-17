// use linalg::Vec3f;

#[derive(Debug)]
pub struct Ray {
    /// The origin of the ray.
    pub o: Vec3f,

    /// The direction of the ray.
    pub d: Vec3f,

    /// Component wise reciprocal of the direction vector.
    pub d_rcp: Vec3f,
}

impl Ray {
    pub fn new(o: Vec3f, d: Vec3f) -> Self {
        let d = d.normalize();
        Ray {
            o,
            d,
            d_rcp: d.recip(),
        }
    }
}
