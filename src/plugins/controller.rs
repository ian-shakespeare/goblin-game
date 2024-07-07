use crate::{
    components::{LookInput, MovementInput},
    constants::{MOUSE_SENSITIVITY_X, MOUSE_SENSITIVITY_Y},
};
use bevy::{
    input::{mouse::MouseMotion, InputSystem},
    prelude::*,
    window::CursorGrabMode,
};

pub struct ControllerPlugin;

impl ControllerPlugin {
    fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
        // Grab cursor
        let mut window = windows.single_mut();
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;

        // Init input resources
        commands.init_resource::<MovementInput>();
        commands.init_resource::<LookInput>();
    }

    fn handle_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut mouse_events: EventReader<MouseMotion>,
        mut movement: ResMut<MovementInput>,
        mut look: ResMut<LookInput>,
    ) {
        if keyboard.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }

        **movement = movement.normalize_or_zero();
        if keyboard.pressed(KeyCode::ShiftLeft) {
            **movement *= 2.0;
        }
        if keyboard.pressed(KeyCode::Space) {
            movement.y = 1.0
        }

        for event in mouse_events.read() {
            look.x -= event.delta.x * MOUSE_SENSITIVITY_X;
            look.y -= event.delta.y * MOUSE_SENSITIVITY_Y;
            look.y = look.y.clamp(-89.9, 89.9);
        }
    }

    fn quit(key: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
        if key.just_pressed(KeyCode::Escape) {
            exit.send(AppExit::Success);
        }
    }
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(PreUpdate, Self::handle_input.after(InputSystem))
            .add_systems(Update, Self::quit);
    }
}
