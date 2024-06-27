use super::{Component, ComponentKind, ComponentValue};
use crate::models::model_manager::ModelId;
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct ModelComponent {
    pub id: ModelId,
    pub tex_coords: [glm::Vec2; 6],
}

impl Component for ModelComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Model
    }

    fn get_value(self) -> super::ComponentValue {
        ComponentValue::Model(self)
    }
}
