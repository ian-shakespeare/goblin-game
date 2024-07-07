use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use goblin_game::plugins::{controller::ControllerPlugin, level::Level, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(Level)
        .add_plugins(ControllerPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
