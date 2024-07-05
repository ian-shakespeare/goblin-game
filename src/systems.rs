use crate::components::{Force, Mass, Velocity};
use bevy::{
    prelude::{Query, Res},
    time::Time,
    transform::components::Transform,
};

pub fn apply_force(mut query: Query<(&mut Velocity, &mut Force, &Mass)>, time: Res<Time>) {
    for (mut velocity, mut force, mass) in &mut query {
        let acceleration = force.0 / mass.0;
        velocity.0 += acceleration * time.delta_seconds();
        force.reset();
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
        transform.translation.z += velocity.0.z * time.delta_seconds();

        // DRAG
        velocity.0 *= 0.8;
    }
}
