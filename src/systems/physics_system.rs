use super::{System, SystemError};
use crate::{
    components::{
        collision::CollisionComponent, gravity::GravityComponent, rigid_body::RigidBodyComponent,
        transform::TransformComponent, ComponentError, ComponentValue,
    },
    constants::GROUND_DRAG,
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

            let transform = ecs.get_component::<TransformComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Transform")),
            )?;

            let gravity = ecs.get_component::<GravityComponent>(*entity).ok_or(
                SystemError::ComponentError(ComponentError::MissingComponent("Gravity")),
            )?;

            let mut new_position = transform.position + rigid_body.velocity;
            let mut new_velocity = rigid_body.velocity + rigid_body.force + gravity.force;

            let up_ray = Ray::new(new_position, Vec3::new(0.0, 1.0, 0.0));
            let down_ray = Ray::new(new_position, Vec3::new(0.0, -1.0, 0.0));
            let north_ray = Ray::new(new_position, Vec3::new(1.0, 0.0, 0.0));
            let south_ray = Ray::new(new_position, Vec3::new(-1.0, 0.0, 0.0));
            let east_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, 1.0));
            let west_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, -1.0));

            let collidable_entities = ecs.get_all_entities_with_component::<CollisionComponent>();
            for collidable_entity in collidable_entities {
                let CollisionComponent { mesh_id } = ecs
                    .get_component::<CollisionComponent>(collidable_entity)
                    .expect("Could not find collision component");

                let collision_transform = ecs
                    .get_component::<TransformComponent>(collidable_entity)
                    .expect("Could not find transform component");

                let collision_mesh = mesh_manager
                    .get_mesh(mesh_id)
                    .expect("Could not find collision mesh");

                // X collisions
                if north_ray.collides(collision_mesh, collision_transform)
                    || south_ray.collides(collision_mesh, collision_transform)
                {
                    new_position = Vec3::new(transform.position.x, new_position.y, new_position.z);
                    new_velocity = Vec3::new(0.0, new_velocity.y, new_velocity.z);
                }

                // Y collisions
                if up_ray.collides(collision_mesh, collision_transform)
                    || down_ray.collides(collision_mesh, collision_transform)
                {
                    new_position = Vec3::new(new_position.x, transform.position.y, new_position.z);
                    new_velocity = GROUND_DRAG * Vec3::new(new_velocity.x, 0.0, new_velocity.z);
                }

                // Z collisions
                if east_ray.collides(collision_mesh, collision_transform)
                    || west_ray.collides(collision_mesh, collision_transform)
                {
                    new_position = Vec3::new(new_position.x, new_position.y, transform.position.z);
                    new_velocity = Vec3::new(new_velocity.x, new_velocity.y, 0.0);
                }
            }

            ecs.set_component(
                *entity,
                ComponentValue::Transform(TransformComponent {
                    position: new_position,
                    rotation: transform.rotation,
                    scale: transform.scale,
                }),
            );

            ecs.set_component(
                *entity,
                ComponentValue::RigidBody(RigidBodyComponent {
                    force: Vec3::zeros(),
                    velocity: new_velocity,
                }),
            );
        }

        Ok(())
    }
}
