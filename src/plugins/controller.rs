use crate::{
    components::{Player, PlayerCam},
    constants::{
        GRAVITY, MOUSE_SENSITIVITY_X, MOUSE_SENSITIVITY_Y, PLAYER_JUMPFORCE, PLAYER_MOVEFORCE,
    },
};
use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    input::{mouse::MouseMotion, ButtonInput},
    math::Vec3,
    prelude::{EventReader, EventWriter, IntoSystemConfigs, KeyCode, Query, Res, With, Without},
    time::Time,
    transform::components::Transform,
    window::{CursorGrabMode, Window},
};
use bevy_rapier3d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::{ExternalForce, RigidBody},
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
    mut player: Query<(&mut ExternalForce, &Transform), With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut force, transform) = player.single_mut();
    let forward = transform
        .rotation
        .mul_vec3(PLAYER_MOVEFORCE * Vec3::new(0.0, 0.0, -1.0))
        .with_y(0.0);

    let backward = transform
        .rotation
        .mul_vec3(PLAYER_MOVEFORCE * Vec3::new(0.0, 0.0, 1.0))
        .with_y(0.0);

    let right = transform
        .rotation
        .mul_vec3(PLAYER_MOVEFORCE * Vec3::new(1.0, 0.0, 0.0))
        .with_y(0.0);

    let left = transform
        .rotation
        .mul_vec3(PLAYER_MOVEFORCE * Vec3::new(-1.0, 0.0, 0.0))
        .with_y(0.0);

    let mut to_move = Vec3::ZERO;

    if key.pressed(KeyCode::KeyW) {
        to_move += forward;
    }
    if key.pressed(KeyCode::KeyS) {
        to_move += backward;
    }
    if key.pressed(KeyCode::KeyA) {
        to_move += left;
    }
    if key.pressed(KeyCode::KeyD) {
        to_move += right;
    }

    to_move = to_move.with_y(0.0);

    if key.just_pressed(KeyCode::Space) {
        to_move += PLAYER_JUMPFORCE * Vec3::new(0.0, 1.0, 0.0);
    }

    force.force = to_move.with_y(0.0) * time.delta_seconds();
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
