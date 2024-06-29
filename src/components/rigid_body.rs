use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct RigidBodyComponent {
    pub acceleration: glm::Vec3,
    pub collision_x_offset: f32,
    pub collision_y_offset: f32,
    pub collision_z_offset: f32,
    pub velocity: glm::Vec3,
}

impl Component for RigidBodyComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::RigidBody
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::RigidBody(self)
    }
}
