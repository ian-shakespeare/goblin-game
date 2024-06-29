use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct CollisionComponent {
    pub normal: glm::Vec3,
    pub position: glm::Vec3,
    pub vertices: [glm::Vec3; 3],
}

impl Component for CollisionComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Collision
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Collision(self)
    }
}
