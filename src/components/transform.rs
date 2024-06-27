use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct TransformComponent {
    pub position: glm::Vec3,
    pub rotation: glm::Vec4,
    pub scale: glm::Vec3,
}

impl Component for TransformComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Transform
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Transform(self)
    }
}
