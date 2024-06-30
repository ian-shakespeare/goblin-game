use super::{Component, ComponentKind, ComponentValue};

#[derive(Clone, Copy, PartialEq)]
pub struct MeshComponent {
    pub id: u32,
}

impl Component for MeshComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Mesh
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Mesh(self)
    }

    fn from_value(value: ComponentValue) -> Self {
        if let ComponentValue::Mesh(mesh) = value {
            return mesh;
        }
        panic!("Invalid mesh component");
    }
}
