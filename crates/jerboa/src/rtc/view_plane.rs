#[derive(Debug, Copy, Clone)]
pub struct ViewPlane {
    /// Horizontal resolution of the view plane.
    h_res: u32,

    /// Vertical resolution of the view plane.
    v_res: u32,

    /// Monitor gamma factor.
    gamma: f32,

    /// Inverse of the monitor gamma factor.
    inv_gamma: f32,
}

impl ViewPlane {
    /// Creates a new view plane with the given horizontal and vertical resolution.
    pub fn new(h_res: u32, v_res: u32, gamma: f32) -> Self {
        ViewPlane { h_res, v_res, gamma, inv_gamma: 1.0 / gamma }
    }

    /// Returns the horizontal resolution of the view plane.
    pub fn h_res(&self) -> u32 {
        self.h_res
    }

    /// Returns the vertical resolution of the view plane.
    pub fn v_res(&self) -> u32 {
        self.v_res
    }
}