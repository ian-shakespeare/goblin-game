use std::{collections::HashMap, rc::Rc};
use crate::resources::Resources;
use super::texture::Texture;

#[derive(Hash, PartialEq, Eq)]
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
    textures: HashMap<TextureId, Rc<Texture>>,
}

impl TextureManager {
    pub fn new(res: &Resources) -> Self {
        let textures = HashMap::from([
            (TextureId::Unknown, Rc::new(Texture::from_resource(res, "textures/unknown.png"))),
            (TextureId::StoneBricks, Rc::new(Texture::from_resource(res, "textures/stone_bricks.png"))),
            (TextureId::WoodPlanks, Rc::new(Texture::from_resource(res, "textures/wood_planks.png"))),
            (TextureId::Dirt, Rc::new(Texture::from_resource(res, "textures/dirt.png"))),
            (TextureId::Grass, Rc::new(Texture::from_resource(res, "textures/grass.png"))),
        ]);

        Self {
            textures,
        }
    }

    pub fn get_texture(&self, id: TextureId) -> Rc<Texture> {
        self.textures.get(&id).cloned().expect("Texture does not exist")
    }
}
