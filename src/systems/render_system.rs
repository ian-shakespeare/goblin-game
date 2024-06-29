use super::SystemError;
use crate::{
    components::{
        mesh::MeshComponent, transform::TransformComponent,
        ComponentError, ComponentValue,
    }, ecs::ECS, entities::Entity, mesh_manager::MeshManager, shader::Shader, utils::degree_to_radian
};
use nalgebra_glm as glm;
use std::{collections::HashSet, sync::Mutex};

pub struct RenderSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
    mesh_manager: MeshManager,
    shader: &'a Shader,
}

impl<'a> RenderSystem<'a> {
    pub fn init(
        ecs: &'a Mutex<ECS>,
        mesh_manager: MeshManager,
        shader: &'a Shader,
    ) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self {
            ecs,
            entities,
            mesh_manager,
            shader,
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

            let mesh_component =
                ecs.get_component::<MeshComponent>(*entity)
                    .ok_or(SystemError::ComponentError(
                        ComponentError::MissingComponent("Model"),
                    ))?;
            let MeshComponent {
                id,
            } = match mesh_component {
                ComponentValue::Mesh(model) => model,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Mesh"),
                    ))
                }
            };
            let mesh = self.mesh_manager.get_mesh(id).expect("Missing mesh");
            mesh
                .draw_instance(self.shader, &model_transform, view_transform, projection_transform)
                .expect("Shader error drawing entity");
        }

        Ok(())
    }
}
