use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm::Vec3;

#[derive(Clone, Copy, PartialEq)]
pub struct RigidBodyComponent {
    pub force: Vec3,
    pub velocity: Vec3,
}

impl Component for RigidBodyComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::RigidBody
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::RigidBody(self)
    }

    fn from_value(value: ComponentValue) -> Self {
        if let ComponentValue::RigidBody(rigid_body) = value {
            return rigid_body;
        }
        panic!("Invalid rigid body component");
    }
}

impl RigidBodyComponent {
    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn apply_drag(&mut self, force: Vec3) {}
}
