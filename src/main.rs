use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use goblin_game::{
    plugins::controller::ControllerPlugin,
    setup::{setup, setup_physics},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(ControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        //        .add_systems(Update, print_ball_altitude)
        .run();
}

fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
    }
}
