use crate::{
    components::{
        model::ModelComponent, texture::TextureComponent, transform::TransformComponent,
        ComponentError, ComponentValue,
    },
    ecs::ECS,
    entities::Entity,
    models::model_manager::ModelManager,
    textures::texture_manager::TextureManager,
    utils::degree_to_radian,
};
use nalgebra_glm as glm;
use std::{collections::HashSet, sync::Mutex};

use super::SystemError;

pub struct RenderSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
    model_manager: ModelManager<'a>,
    texture_manager: TextureManager,
}

impl<'a> RenderSystem<'a> {
    pub fn init(
        ecs: &'a Mutex<ECS>,
        model_manager: ModelManager<'a>,
        texture_manager: TextureManager,
    ) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self {
            ecs,
            entities,
            model_manager,
            texture_manager,
        })
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn draw(
        &mut self,
        view_transform: &glm::Mat4,
        projection_transform: &glm::Mat4,
    ) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;
        for entity in &self.entities {
            let transform_component = ecs.get_component::<TransformComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Transform")),
            )?;
            let TransformComponent {
                position,
                rotation,
                scale,
            } = match transform_component {
                ComponentValue::Transform(transform) => transform,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Transform"),
                    ))
                }
            };
            let mut model_transform = glm::Mat4::identity();
            model_transform = glm::translate(&model_transform, &position);
            if rotation[0] >= 0.0 {
                model_transform = glm::rotate(
                    &model_transform,
                    degree_to_radian(rotation[0]),
                    &glm::Vec3::new(rotation[1], rotation[2], rotation[3]),
                );
            }
            model_transform = glm::scale(&model_transform, &scale);

            let texture_component = ecs.get_component::<TextureComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Texture")),
            )?;
            let TextureComponent { id } = match texture_component {
                ComponentValue::Texture(texture) => texture,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Texture"),
                    ))
                }
            };
            let texture = self.texture_manager.get_texture(id);
            texture.bind();

            let model_component =
                ecs.get_component::<ModelComponent>(*entity)
                    .ok_or(SystemError::ComponentError(
                        ComponentError::MissingComponent("Model"),
                    ))?;
            let ModelComponent { id, tex_coords: _ } = match model_component {
                ComponentValue::Model(model) => model,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Model"),
                    ))
                }
            };
            let model = self.model_manager.get_model(id);
            model
                .draw_instance(&model_transform, view_transform, projection_transform)
                .expect("Shader error drawing entity");
        }

        Ok(())
    }
}
