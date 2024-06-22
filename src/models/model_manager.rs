use std::{collections::HashMap, rc::Rc};
use crate::shader::Shader;
use super::{cube::CUBE_VERTICES, model::Model, plane::PLANE_VERTICES};

const DEFAULT_TEX_COORDS: [(f32, f32);6] = [
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 1.0),
];

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum ModelId {
    Unknown = 0x00,
    Plane = 0x01,
    Cube = 0x02,
}

impl From<u32> for ModelId {
    fn from(value: u32) -> Self {
        match value {
            0x01 => ModelId::Plane,
            0x02 => ModelId::Cube,
            _ => ModelId::Unknown,
        }
    }
}

pub struct ModelManager<'a> {
    models: HashMap<ModelId, Rc<Model<'a>>>,
}

impl<'a> ModelManager<'a> {
    pub fn new(shader: &'a Shader) -> Self {
        let models = HashMap::from([
            (
                ModelId::Unknown,
                Rc::new(
                    Model::from_vertices(
                        shader,
                        CUBE_VERTICES
                            .iter()
                            .enumerate()
                            .map(|(i, points)| (*points, DEFAULT_TEX_COORDS[i % 6]).into())
                            .collect(),
                    ),
                )
            ),
            (
                ModelId::Plane,
                Rc::new(
                    Model::from_vertices(
                        shader,
                        PLANE_VERTICES
                            .iter()
                            .enumerate()
                            .map(|(i, points)| (*points, DEFAULT_TEX_COORDS[i % 6]).into())
                            .collect(),
                    ),
                ),
            ),
            (
                ModelId::Cube,
                Rc::new(
                    Model::from_vertices(
                        shader,
                        CUBE_VERTICES
                            .iter()
                            .enumerate()
                            .map(|(i, points)| (*points, DEFAULT_TEX_COORDS[i % 6]).into())
                            .collect(),
                    ),
                ),
            ),
        ]);

        Self {
            models,
        }
    }

    pub fn get_model(&self, id: ModelId) -> Rc<Model> {
        self.models.get(&id).cloned().expect("Model does not exist")
    }
}
