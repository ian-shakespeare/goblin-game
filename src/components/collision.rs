use super::{Component, ComponentKind, ComponentValue};

#[derive(Clone, Copy, PartialEq)]
pub struct CollisionComponent {
    pub mesh_id: u32,
}

impl Component for CollisionComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Collision
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Collision(self)
    }
}
