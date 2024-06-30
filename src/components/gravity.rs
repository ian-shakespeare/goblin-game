use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct GravityComponent {
    pub force: glm::Vec3,
}

impl<'a> Component for GravityComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Gravity
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Gravity(self)
    }

    fn from_value(value: ComponentValue) -> Self {
        if let ComponentValue::Gravity(gravity) = value {
            return gravity;
        }
        panic!("Invalid gravity component");
    }
}
