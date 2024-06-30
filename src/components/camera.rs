use nalgebra_glm::Vec3;

use super::{Component, ComponentKind, ComponentValue};

#[derive(Clone, Copy, PartialEq)]
pub struct CameraComponent {
    pub front: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
}

impl Component for CameraComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Camera
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Camera(self)
    }

    fn from_value(value: ComponentValue) -> Self {
        if let ComponentValue::Camera(camera) = value {
            return camera;
        }
        panic!("Invalid camera component");
    }
}
