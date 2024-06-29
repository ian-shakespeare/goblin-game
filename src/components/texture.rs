use super::{Component, ComponentKind, ComponentValue};
use crate::textures::texture_manager::TextureId;

#[derive(Clone, Copy, PartialEq)]
pub struct TextureComponent {
    pub id: TextureId,
}

impl Component for TextureComponent {
    fn get_kind() -> ComponentKind {
        ComponentKind::Texture
    }

    fn get_value(self) -> ComponentValue {
        ComponentValue::Texture(self)
    }
}
