use std::u32;
use nalgebra_glm as glm;
use crate::{entities::Entity, polygons::{cube::Cube, plane::Plane, Polygon}, resources::Resources, shader::Shader, textures::texture_manager::TextureManager};

#[derive(Debug)]
pub enum LevelError {
    CouldNotLoad,
    MissingEntityType,
    InvalidEntityType,
    MissingOperationType,
    MissingArgument,
    InvalidArgument,
    MissingTexture,
}

pub struct Level<'a> {
    entities: Vec<Entity<'a>>,
}

impl<'a> Level<'a> {
    fn parse_argument(args: &Vec<&str>, index: usize) -> Result<f32, LevelError> {
        match args.get(index) {
            Some(arg) => {
                let parsed = arg.trim().parse::<f32>().map_err(|_| LevelError::InvalidArgument);

                match parsed {
                    Ok(value) => Ok(value),
                    Err(e) => {
                        eprintln!("Could not parse argument: \"{}\"", arg);
                        Err(e)
                    },
                }
            },
            None => Err(LevelError::MissingArgument),
        }
    }

    fn block_to_entity(block: &'_ str, shader: &'a Shader, texture_manager: &'a TextureManager) -> Result<Entity<'a>, LevelError> {
        let mut lines = block.split("\n");
        let render_token = lines.next();
        let texture_line = match lines.next() {
            Some(value) => value.split(" "),
            None => return Err(LevelError::MissingTexture),
        };
        let texture_tokens: Vec<&str> = texture_line.collect();
        let texture_id = match texture_tokens.get(1) {
            Some(token) => match u32::from_str_radix(token.replace("0x", "").trim(), 16) {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Could not parse texture id: {}", token);
                    return Err(LevelError::InvalidArgument)
                },
            },
            None => 0,
        };
        let texture_scale_x = Self::parse_argument(&texture_tokens, 2).unwrap_or(1.0);
        let texture_scale_y = Self::parse_argument(&texture_tokens, 3).unwrap_or(1.0);

        let renderer = match render_token {
            Some(s) => match s {
                "plane" => Plane::with_scaled_tex(shader, texture_scale_x, texture_scale_y),
                "cube" => Cube::with_scaled_tex(shader, texture_scale_x, texture_scale_y),
                _ => return Err(LevelError::InvalidEntityType),
            },
            None => return Err(LevelError::MissingEntityType),
        };
        let texture = texture_manager.get_texture(texture_id.into());
        let mut entity = Entity::new(renderer, texture);

        for line in lines {
            let tokens: Vec<&str> = line.split(" ").collect();
            match tokens.get(0) {
                Some(token) => match token.trim() {
                    "trans" => {
                        let x = Self::parse_argument(&tokens, 1)?;
                        let y = Self::parse_argument(&tokens, 2)?;
                        let z = Self::parse_argument(&tokens, 3)?;
                        entity.translate(glm::Vec3::new(x, y, z));
                    },
                    "rot" => {
                        let angle = Self::parse_argument(&tokens, 1)?;
                        let x = Self::parse_argument(&tokens, 2)?;
                        let y = Self::parse_argument(&tokens, 3)?;
                        let z = Self::parse_argument(&tokens, 4)?;
                        entity.rotate(angle, glm::Vec3::new(x, y, z));
                    },
                    "scale" => {
                        let x = Self::parse_argument(&tokens, 1)?;
                        let y = Self::parse_argument(&tokens, 2)?;
                        let z = Self::parse_argument(&tokens, 3)?;
                        entity.scale(glm::Vec3::new(x, y, z));
                    },
                    _ => (),
                },
                None => return Err(LevelError::MissingOperationType),
            }
        }

        Ok(entity)
    }

    pub fn new(entities: Vec<Entity<'a>>) -> Self {
        Self {
            entities,
        }
    }

    pub fn from_resource(res: &Resources, name: &str, shader: &'a Shader, texture_manager: &'a TextureManager) -> Result<Self, LevelError> {
        let level = res.load_string(name).map_err(|_| LevelError::CouldNotLoad)?;

        let mut entities: Vec<Entity<'a>> = Vec::new();

        for block in level.split("\n\n") {
            entities.push(Self::block_to_entity(block, shader, texture_manager)?);
        }

        Ok(Self {
            entities,
        })
    }

    pub fn draw(&self, view_transform: &glm::Mat4, projection_transform: &glm::Mat4) {
        for entity in &self.entities {
            entity.draw(view_transform, projection_transform);
        }
    }
}
