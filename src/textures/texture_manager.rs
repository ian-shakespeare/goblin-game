use super::texture::Texture;
use crate::resources::Resources;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum TextureId {
    Unknown = 0x00,
    StoneBricks = 0x01,
    WoodPlanks = 0x02,
    Dirt = 0x03,
    Grass = 0x04,
}

impl From<u32> for TextureId {
    fn from(value: u32) -> Self {
        match value {
            0x01 => TextureId::StoneBricks,
            0x02 => TextureId::WoodPlanks,
            0x03 => TextureId::Dirt,
            0x04 => TextureId::Grass,
            _ => TextureId::Unknown,
        }
    }
}

pub struct TextureManager {
    textures: HashMap<TextureId, Texture>,
}

impl TextureManager {
    pub fn new(res: &Resources) -> Self {
        let textures = HashMap::from([
            (
                TextureId::Unknown,
                Texture::from_resource(res, "textures/unknown.png"),
            ),
            (
                TextureId::StoneBricks,
                Texture::from_resource(res, "textures/stone_bricks.png"),
            ),
            (
                TextureId::WoodPlanks,
                Texture::from_resource(res, "textures/wood_planks.png"),
            ),
            (
                TextureId::Dirt,
                Texture::from_resource(res, "textures/dirt.png"),
            ),
            (
                TextureId::Grass,
                Texture::from_resource(res, "textures/grass.png"),
            ),
        ]);

        Self { textures }
    }

    pub fn get_texture(&self, id: TextureId) -> Texture {
        self.textures
            .get(&id)
            .cloned()
            .expect("Texture does not exist")
    }
}
