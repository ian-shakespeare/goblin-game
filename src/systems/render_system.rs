use super::{System, SystemError};
use crate::{
    components::{
        camera::CameraComponent, mesh::MeshComponent, transform::TransformComponent,
        ComponentError, ComponentValue,
    },
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    ecs::ECS,
    entities::Entity,
    mesh_manager::MeshManager,
    shader::Shader,
    utils::degree_to_radian,
};
use nalgebra_glm::{self as glm, Mat4, Vec3};
use std::{collections::HashSet, sync::Mutex};

pub struct RenderSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
    mesh_manager: &'a Mutex<MeshManager>,
    shader: &'a Shader,
    camera_entity: Entity,
}

impl<'a> RenderSystem<'a> {
    pub fn init(
        ecs: &'a Mutex<ECS>,
        mesh_manager: &'a Mutex<MeshManager>,
        shader: &'a Shader,
        camera_entity: Entity,
    ) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self {
            ecs,
            entities,
            mesh_manager,
            shader,
            camera_entity,
        })
    }
}

impl<'a> System for RenderSystem<'a> {
    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn update(&mut self) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;
        let mesh_manager = self
            .mesh_manager
            .lock()
            .map_err(|_| SystemError::LockError)?;

        let camera = match ecs
            .get_component::<CameraComponent>(self.camera_entity)
            .expect("Could not get camera component")
        {
            ComponentValue::Camera(camera) => camera,
            _ => panic!("Invalid camera component"),
        };
        let camera_transform = match ecs
            .get_component::<TransformComponent>(self.camera_entity)
            .expect("Missing camera transform component")
        {
            ComponentValue::Transform(camera) => camera,
            _ => panic!("Invalid camera transform"),
        };
        let view_transform = glm::look_at(
            &camera_transform.position,
            &(camera_transform.position + camera.front),
            &camera.up,
        );
        let projection_transform = glm::perspective::<f32>(
            SCREEN_WIDTH / SCREEN_HEIGHT,
            degree_to_radian(camera.fov),
            0.1,
            100.0,
        );

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

            let mut model_transform = Mat4::identity();
            model_transform = glm::translate(&model_transform, &position);
            if let Some(rotation) = rotation {
                model_transform = glm::rotate(
                    &model_transform,
                    degree_to_radian(rotation[0]),
                    &Vec3::new(rotation[1], rotation[2], rotation[3]),
                );
            }
            if let Some(scale) = scale {
                model_transform = glm::scale(&model_transform, &scale);
            }

            let mesh_component =
                ecs.get_component::<MeshComponent>(*entity)
                    .ok_or(SystemError::ComponentError(
                        ComponentError::MissingComponent("Model"),
                    ))?;
            let MeshComponent { id } = match mesh_component {
                ComponentValue::Mesh(model) => model,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Mesh"),
                    ))
                }
            };
            let mesh = mesh_manager.get_mesh(id).expect("Missing mesh");
            mesh.draw_instance(
                self.shader,
                &model_transform,
                &view_transform,
                &projection_transform,
            )
            .expect("Shader error drawing entity");
        }

        Ok(())
    }
}
