use crate::{
    components::{Force, Player, PlayerCam},
    constants::{MOUSE_SENSITIVITY_X, MOUSE_SENSITIVITY_Y, PLAYER_MOVESPEED},
};
use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    input::{mouse::MouseMotion, ButtonInput},
    math::Vec3,
    prelude::{EventReader, EventWriter, IntoSystemConfigs, KeyCode, Query, Res, With, Without},
    transform::components::Transform,
    window::{CursorGrabMode, Window},
};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, grab_mouse).add_systems(
            Update,
            (quit, ((rotate_player, move_player), sync_camera).chain()),
        );
    }
}

fn grab_mouse(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn rotate_player(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * MOUSE_SENSITIVITY_X;
        let pitch = -motion.delta.y * MOUSE_SENSITIVITY_Y;
        transform.rotate_y(yaw.clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2));
        transform.rotate_local_x(pitch);
    }
}

fn move_player(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Force, &Transform), With<Player>>,
) {
    let (mut player_force, player_transform) = player.single_mut();

    let forward_force = player_transform
        .rotation
        .mul_vec3(PLAYER_MOVESPEED * Vec3::new(0.0, 0.0, -1.0));

    let backward_force = player_transform
        .rotation
        .mul_vec3(PLAYER_MOVESPEED * Vec3::new(0.0, 0.0, 1.0));

    let right_force = player_transform
        .rotation
        .mul_vec3(PLAYER_MOVESPEED * Vec3::new(1.0, 0.0, 0.0));

    let left_force = player_transform
        .rotation
        .mul_vec3(PLAYER_MOVESPEED * Vec3::new(-1.0, 0.0, 0.0));

    if key.pressed(KeyCode::KeyW) {
        player_force.apply_force(forward_force);
    }
    if key.pressed(KeyCode::KeyS) {
        player_force.apply_force(backward_force);
    }
    if key.pressed(KeyCode::KeyA) {
        player_force.apply_force(left_force);
    }
    if key.pressed(KeyCode::KeyD) {
        player_force.apply_force(right_force);
    }
}

fn sync_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &PlayerCam), Without<Player>>,
) {
    let player_transform = player.single();
    let (mut camera_transform, _) = camera.single_mut();

    camera_transform.translation = player_transform.translation;
    camera_transform.rotation = player_transform.rotation;
}

fn quit(key: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if key.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
