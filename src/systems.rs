use bevy::{
    math::Vec3,
    prelude::{Query, Res},
    time::Time,
};
use bevy_rapier3d::control::KinematicCharacterController;

pub fn update_physics(time: Res<Time>, mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(1.0, -5.0, -1.0) * time.delta_seconds());
    }
}
