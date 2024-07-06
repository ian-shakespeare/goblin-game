use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use goblin_game::{
    plugins::controller::ControllerPlugin,
    setup::{setup, setup_physics},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(ControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .run();
}
