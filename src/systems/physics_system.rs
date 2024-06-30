use super::{System, SystemError};
use crate::{
    components::{
        collision::CollisionComponent, gravity::GravityComponent, rigid_body::RigidBodyComponent,
        transform::TransformComponent, ComponentError, ComponentValue,
    },
    constants::MAX_PLAYER_VELOCITY,
    ecs::ECS,
    entities::Entity,
    mesh_manager::MeshManager,
    ray::Ray,
};
use nalgebra_glm::Vec3;
use std::{collections::HashSet, sync::Mutex};

pub struct PhysicsSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
    mesh_manager: &'a Mutex<MeshManager>,
}

impl<'a> PhysicsSystem<'a> {
    pub fn init(
        ecs: &'a Mutex<ECS>,
        mesh_manager: &'a Mutex<MeshManager>,
    ) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self {
            ecs,
            entities,
            mesh_manager,
        })
    }
}

impl<'a> System for PhysicsSystem<'a> {
    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn update(&mut self) -> Result<(), SystemError> {
        let mut ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;
        let mesh_manager = self
            .mesh_manager
            .lock()
            .map_err(|_| SystemError::LockError)?;

        for entity in &self.entities {
            let rigid_body = ecs.get_component::<RigidBodyComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("RigidBody")),
            )?;
            let RigidBodyComponent {
                mut acceleration,
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
            let mut new_velocity = velocity + acceleration + force;
            new_velocity = Vec3::new(
                new_velocity.x.min(MAX_PLAYER_VELOCITY),
                new_velocity.y.min(MAX_PLAYER_VELOCITY),
                new_velocity.z.min(MAX_PLAYER_VELOCITY),
            );

            let up_ray = Ray::new(new_position, Vec3::new(0.0, 1.0, 0.0));
            let down_ray = Ray::new(new_position, Vec3::new(0.0, -1.0, 0.0));
            let north_ray = Ray::new(new_position, Vec3::new(1.0, 0.0, 0.0));
            let south_ray = Ray::new(new_position, Vec3::new(-1.0, 0.0, 0.0));
            let east_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, 1.0));
            let west_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, -1.0));

            let collidable_entities = ecs.get_all_entities_with_component::<CollisionComponent>();
            for collidable_entity in collidable_entities {
                let collision = ecs
                    .get_component::<CollisionComponent>(collidable_entity)
                    .expect("Could not find collision component");
                let collision_transform = match ecs
                    .get_component::<TransformComponent>(collidable_entity)
                    .expect("Could not find collidable's transform component")
                {
                    ComponentValue::Transform(transform) => transform,
                    _ => panic!("Could not match collidable's transform"),
                };

                if let ComponentValue::Collision(CollisionComponent { mesh_id }) = collision {
                    let collision_mesh = mesh_manager
                        .get_mesh(mesh_id)
                        .expect("Could not find collision mesh");

                    // X collisions
                    if north_ray.collides(collision_mesh, collision_transform, collision_x_offset)
                        || south_ray.collides(
                            collision_mesh,
                            collision_transform,
                            collision_x_offset,
                        )
                    {
                        new_position = Vec3::new(position.x, new_position.y, new_position.z);
                        new_velocity = Vec3::new(0.0, new_velocity.y, new_velocity.z);
                        acceleration = Vec3::new(0.0, acceleration.y, acceleration.z);
                    }

                    // Y collisions
                    if up_ray.collides(collision_mesh, collision_transform, collision_y_offset)
                        || down_ray.collides(
                            collision_mesh,
                            collision_transform,
                            collision_y_offset,
                        )
                    {
                        new_position = Vec3::new(new_position.x, position.y, new_position.z);
                        new_velocity = Vec3::new(new_velocity.x, 0.0, new_velocity.z);
                        acceleration = Vec3::new(acceleration.x, 0.0, acceleration.z);
                    }

                    // Z collisions
                    if east_ray.collides(collision_mesh, collision_transform, collision_z_offset)
                        || west_ray.collides(
                            collision_mesh,
                            collision_transform,
                            collision_z_offset,
                        )
                    {
                        new_position = Vec3::new(new_position.x, new_position.y, position.z);
                        new_velocity = Vec3::new(new_velocity.x, new_velocity.y, 0.0);
                        acceleration = Vec3::new(acceleration.x, acceleration.y, 0.0);
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
