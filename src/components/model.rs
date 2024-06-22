use nalgebra_glm as glm;
use crate::models::model_manager::ModelId;

#[derive(Clone, Copy, PartialEq)]
pub struct ModelComponent {
    pub id: ModelId,
    pub tex_coords: [glm::Vec2;6],
}
