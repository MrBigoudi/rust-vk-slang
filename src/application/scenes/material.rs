use glam::Vec4;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Vec4,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec4::from_array([1., 1., 1., 1.]),
        }
    }
}
