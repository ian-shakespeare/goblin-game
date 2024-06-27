use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct GravityComponent {
    pub force: glm::Vec3,
}

impl Component for GravityComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Gravity
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Gravity(self)
    }
}
