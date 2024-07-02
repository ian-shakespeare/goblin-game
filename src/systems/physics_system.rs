use super::{System, SystemError};
use crate::{
    collider::Collider,
    components::{
        collidable::Collidable, gravity::GravityComponent, rigid_body::RigidBody,
        transform::Transform,
    },
    constants::GROUND_DRAG,
    ecs::Ecs,
    ray::Ray,
    utils::flatten_vector,
};
use nalgebra_glm::Vec3;
use std::sync::Mutex;

pub struct PhysicsSystem<'a> {
    ecs: &'a Mutex<Ecs>,
    collider: &'a mut Collider,
}

impl<'a> PhysicsSystem<'a> {
    pub fn init(ecs: &'a Mutex<Ecs>, collider: &'a mut Collider) -> Self {
        Self { ecs, collider }
    }
}

impl<'a> System for PhysicsSystem<'a> {
    fn update(&mut self) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;

        let mut rigid_bodies = ecs
            .get_component_vec::<RigidBody>()
            .expect("Could not get component vector");
        let mut transforms = ecs
            .get_component_vec::<Transform>()
            .expect("Could not get component vector");
        let mut gravities = ecs
            .get_component_vec::<GravityComponent>()
            .expect("Could not get component vector");

        let union = rigid_bodies
            .iter_mut()
            .zip(transforms.iter_mut().zip(gravities.iter_mut()))
            .filter_map(|(rigid_body, (transform, gravity))| {
                Some((rigid_body.as_mut()?, transform.as_mut()?, gravity.as_mut()?))
            });

        let collidables = ecs
            .get_component_vec::<Collidable>()
            .expect("Could not get component vector");

        let collidables: Vec<(&Collidable, &Transform)> = collidables
            .iter()
            .zip(transforms.iter())
            .filter_map(|(collidable, transform)| Some((collidable.as_ref()?, transform.as_ref()?)))
            .collect();

        for (rigid_body, transform, gravity) in union {
            let mut new_position = transform.position() + rigid_body.velocity();
            let mut new_velocity = rigid_body.velocity() + rigid_body.net_force() + gravity.force;

            let up_ray = Ray::new(new_position, Vec3::new(0.0, 1.0, 0.0));
            let down_ray = Ray::new(new_position, Vec3::new(0.0, -1.0, 0.0));
            let north_ray = Ray::new(new_position, Vec3::new(1.0, 0.0, 0.0));
            let south_ray = Ray::new(new_position, Vec3::new(-1.0, 0.0, 0.0));
            let east_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, 1.0));
            let west_ray = Ray::new(new_position, Vec3::new(0.0, 0.0, -1.0));

            for (collidable, collidable_transform) in &collidables {
                let hitbox = collidable.hitbox();
                // X collisions
                if self
                    .collider
                    .collides(&north_ray, hitbox, collidable_transform)
                    || self
                        .collider
                        .collides(&south_ray, hitbox, collidable_transform)
                {
                    new_position =
                        Vec3::new(transform.position().x, new_position.y, new_position.z);
                    new_velocity = Vec3::new(0.0, new_velocity.y, new_velocity.z);
                }

                // Y collisions
                if self
                    .collider
                    .collides(&up_ray, hitbox, collidable_transform)
                    || self
                        .collider
                        .collides(&down_ray, hitbox, collidable_transform)
                {
                    new_position =
                        Vec3::new(new_position.x, transform.position().y, new_position.z);
                    new_velocity = GROUND_DRAG * flatten_vector(new_velocity);
                }

                // Z collisions
                if self
                    .collider
                    .collides(&east_ray, hitbox, collidable_transform)
                    || self
                        .collider
                        .collides(&west_ray, hitbox, collidable_transform)
                {
                    new_position =
                        Vec3::new(new_position.x, new_position.y, transform.position().z);
                    new_velocity = Vec3::new(new_velocity.x, new_velocity.y, 0.0);
                }
            }

            transform.translate(new_position);
            // TODO: Groud drag should only happen when on the ground.
            rigid_body.set_velocity(new_velocity);
            rigid_body.reset_force();
        }

        Ok(())
    }
}
