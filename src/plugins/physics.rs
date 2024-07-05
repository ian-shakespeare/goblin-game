use crate::systems::{apply_force, apply_velocity};
use bevy::{
    app::{App, Plugin, Update},
    prelude::IntoSystemConfigs,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_force, apply_velocity).chain());
    }
}
