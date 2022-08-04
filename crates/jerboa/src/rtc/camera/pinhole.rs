pub enum ProjectionKind {
    Orthographic,
    Perspective,
}

pub struct PinholeCamera {
    pub camera_to_screen: glam::Mat4,
    pub screen_to_raster: glam::Mat4,
    pub raster_to_camera: glam::Mat4,
    pub raster_to_screen: glam::Mat4,
}