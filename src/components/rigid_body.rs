use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct RigidBodyComponent {
    pub velocity: glm::Vec3,
    pub acceleration: glm::Vec3,
}

impl Component for RigidBodyComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::RigidBody
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::RigidBody(self)
    }
}
