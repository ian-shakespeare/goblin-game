use super::SystemError;
use crate::{
    components::{
        collision::CollisionComponent, gravity::GravityComponent, rigid_body::RigidBodyComponent,
        transform::TransformComponent, ComponentError, ComponentValue,
    },
    ecs::ECS,
    entities::Entity,
    ray::Ray,
};
use nalgebra_glm as glm;
use std::{collections::HashSet, sync::Mutex};

pub struct PhysicsSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
}

impl<'a> PhysicsSystem<'a> {
    pub fn init(ecs: &'a Mutex<ECS>) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self { ecs, entities })
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn update(&mut self) -> Result<(), SystemError> {
        let mut ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;

        for entity in &self.entities {
            let rigid_body = ecs.get_component::<RigidBodyComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("RigidBody")),
            )?;
            let RigidBodyComponent {
                acceleration,
                collision_x_offset,
                collision_y_offset,
                collision_z_offset,
                velocity,
            } = match rigid_body {
                ComponentValue::RigidBody(rigid_body) => rigid_body,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Gravity"),
                    ))
                }
            };

            let transform = ecs.get_component::<TransformComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Transform")),
            )?;
            let TransformComponent {
                position,
                rotation,
                scale,
            } = match transform {
                ComponentValue::Transform(transform) => transform,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Transform"),
                    ))
                }
            };

            let gravity = ecs.get_component::<GravityComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Gravity")),
            )?;
            let GravityComponent { force } = match gravity {
                ComponentValue::Gravity(gravity) => gravity,
                _ => {
                    return Err(SystemError::ComponentError(
                        ComponentError::MissingComponent("Gravity"),
                    ))
                }
            };

            let mut new_position = position + velocity;
            let mut new_velocity = velocity + force;

            let up_ray = Ray::new(new_position, glm::Vec3::new(0.0, 1.0, 0.0));
            let down_ray = Ray::new(new_position, glm::Vec3::new(0.0, -1.0, 0.0));
            let north_ray = Ray::new(new_position, glm::Vec3::new(1.0, 0.0, 0.0));
            let south_ray = Ray::new(new_position, glm::Vec3::new(-1.0, 0.0, 0.0));
            let east_ray = Ray::new(new_position, glm::Vec3::new(0.0, 0.0, 1.0));
            let west_ray = Ray::new(new_position, glm::Vec3::new(0.0, 0.0, -1.0));

            let all_collisions = ecs.get_all_components::<CollisionComponent>();
            for collision in all_collisions {
                if let ComponentValue::Collision(plane) = collision {
                    // X collisions
                    if north_ray.collides(plane, collision_x_offset)
                        || south_ray.collides(plane, collision_x_offset)
                    {
                        new_position = glm::Vec3::new(position.x, new_position.y, new_position.z);
                        new_velocity = glm::Vec3::new(0.0, new_velocity.y, new_velocity.z);
                    }

                    // Y collisions
                    if up_ray.collides(plane, collision_y_offset)
                        || down_ray.collides(plane, collision_y_offset)
                    {
                        new_position = glm::Vec3::new(new_position.x, position.y, new_position.z);
                        new_velocity = glm::Vec3::new(new_velocity.x, 0.0, new_velocity.z);
                    }

                    // Z collisions
                    if east_ray.collides(plane, collision_z_offset)
                        || west_ray.collides(plane, collision_z_offset)
                    {
                        new_position = glm::Vec3::new(new_position.x, new_position.y, position.z);
                        new_velocity = glm::Vec3::new(new_velocity.x, new_velocity.y, 0.0);
                    }
                }
            }

            ecs.set_component(
                *entity,
                ComponentValue::Transform(TransformComponent {
                    position: new_position,
                    rotation,
                    scale,
                }),
            );

            ecs.set_component(
                *entity,
                ComponentValue::RigidBody(RigidBodyComponent {
                    acceleration,
                    collision_x_offset,
                    collision_y_offset,
                    collision_z_offset,
                    velocity: new_velocity,
                }),
            );
        }

        Ok(())
    }
}
