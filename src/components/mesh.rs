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
}
