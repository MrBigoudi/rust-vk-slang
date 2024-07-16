use glam::Vec4;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub p0: Vec4,
    pub p1: Vec4,
    pub p2: Vec4,
    pub model_index: usize,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            p0: Vec4::from_array([-1., 0., 0., 1.]),
            p1: Vec4::from_array([1., 0., 0., 1.]),
            p2: Vec4::from_array([0., 1., 0., 1.]),
            model_index: 0,
        }
    }
}
