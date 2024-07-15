use glam::Mat4;

use super::triangle::Triangle;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub model_matrix: Mat4,
    pub material_index: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            model_matrix: Mat4::IDENTITY,
            material_index: 0,
        }
    }
}

impl Model {
    pub fn triangle() -> (Self, Vec<Triangle>) {
        (Model::default(), vec![Triangle::default()])
    }

    pub fn from_tobj(model: &tobj::Model) -> (Self, Vec<Triangle>) {
        // TODO:
        todo!("{:?}", model)
    }
}
