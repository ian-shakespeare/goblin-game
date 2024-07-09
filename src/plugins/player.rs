use bevy::{prelude::*, render::mesh::PrimitiveTopology};
use bevy_rapier3d::prelude::*;

use crate::{
    components::{LineMaterial, LineStrip, LookInput, MovementInput, Player, PlayerActions},
    constants::{
        GRAVITY, GROUND_TIMER, PLAYER_HEIGHT, PLAYER_JUMP_SPEED, PLAYER_MASS, PLAYER_MOVE_SPEED,
        PLAYER_RADIUS,
    },
};

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn setup(mut commands: Commands) {
        commands
            .spawn((
                Player,
                SpatialBundle {
                    transform: Transform::from_xyz(-2.0, 5.0, 0.0),
                    ..default()
                },
                Collider::cuboid(PLAYER_RADIUS, PLAYER_HEIGHT / 2.0, PLAYER_RADIUS),
                KinematicCharacterController {
                    custom_mass: Some(PLAYER_MASS),
                    up: Vec3::Y,
                    offset: CharacterLength::Absolute(0.01),
                    slide: true,
                    autostep: Some(CharacterAutostep {
                        max_height: CharacterLength::Relative(0.3),
                        min_width: CharacterLength::Relative(0.5),
                        include_dynamic_bodies: false,
                    }),
                    max_slope_climb_angle: 45.0_f32.to_radians(),
                    apply_impulse_to_dynamic_bodies: true,
                    snap_to_ground: None,
                    ..default()
                },
            ))
            .with_children(|builder| {
                // Camera
                builder.spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, PLAYER_HEIGHT / 4.0, 0.0),
                    ..default()
                });
            });
    }

    fn move_player(
        time: Res<Time>,
        mut input: ResMut<MovementInput>,
        mut player: Query<
            (
                &mut Transform,
                &mut KinematicCharacterController,
                Option<&KinematicCharacterControllerOutput>,
            ),
            With<Player>,
        >,
        mut vertical_movement: Local<f32>,
        mut grounded_timer: Local<f32>,
    ) {
        let delta_time = time.delta_seconds();

        let Ok((transform, mut controller, output)) = player.get_single_mut() else {
            return;
        };

        let mut movement = PLAYER_MOVE_SPEED * Vec3::new(input.x, 0.0, input.z);
        let jump_speed = input.y * PLAYER_JUMP_SPEED;

        **input = Vec3::ZERO;

        if output.map(|output| output.grounded).unwrap_or(false) {
            *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        }

        if *grounded_timer > 0.0 {
            *grounded_timer -= delta_time;

            if jump_speed > 0.0 {
                *vertical_movement = jump_speed;
                *grounded_timer = 0.0;
            }
        }

        movement.y = *vertical_movement;
        *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
        controller.translation = Some(transform.rotation * (movement * delta_time));
    }

    fn look_player(
        input: Res<LookInput>,
        mut player: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
        mut camera: Query<&mut Transform, With<Camera>>,
    ) {
        let Ok(mut transform) = player.get_single_mut() else {
            return;
        };
        transform.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());

        let Ok(mut transform) = camera.get_single_mut() else {
            return;
        };
        transform.rotation = Quat::from_axis_angle(Vec3::X, input.y.to_radians());
    }

    fn perform_actions(
        mut input: ResMut<PlayerActions>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut line_materials: ResMut<Assets<LineMaterial>>,
        context: Res<RapierContext>,
        player_transform: Query<&Transform, With<Player>>,
        camera_transform: Query<&Transform, With<Camera>>,
    ) {
        let Ok(player_transform) = player_transform.get_single() else {
            return;
        };
        let Ok(camera_transform) = camera_transform.get_single() else {
            return;
        };
        // Do actions
        if input.shoot {
            println!("Shoot");
            input.shoot = false;
            let translation = player_transform.translation + camera_transform.translation;
            let rotation = player_transform.rotation + camera_transform.rotation;
            let ray_origin = translation;
            let ray_direction = rotation.mul_vec3(-Vec3::Z);
            let max_time_to_impact = 2.0;
            /* let hit = context.cast_ray(
                ray_origin,
                ray_direction,
                max_time_to_impact,
                false,
                QueryFilter::only_dynamic(),
            ); */

            let red: f32 = rand::random();
            let green: f32 = rand::random();
            let blue: f32 = rand::random();
            commands.spawn(MaterialMeshBundle {
                mesh: meshes.add(LineStrip {
                    points: vec![Vec3::ZERO, -50.0 * Vec3::Z],
                }),
                transform: Transform::from_translation(translation).with_rotation(rotation),
                material: line_materials.add(LineMaterial {
                    color: LinearRgba::new(red, green, blue, 1.0),
                }),
                ..default()
            });
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::look_player)
            .add_systems(FixedUpdate, Self::move_player)
            .add_systems(FixedUpdate, Self::perform_actions);
    }
}
