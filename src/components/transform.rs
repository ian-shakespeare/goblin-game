use super::{Component, ComponentKind, ComponentValue};
use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct TransformComponent {
    pub position: glm::Vec3,
    pub rotation: Option<glm::Vec4>,
    pub scale: Option<glm::Vec3>,
}

impl Component for TransformComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Transform
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Transform(self)
    }

    fn from_value(value: ComponentValue) -> Self {
        if let ComponentValue::Transform(transform) = value {
            return transform;
        }
        panic!("Invalid transform component");
    }
}
