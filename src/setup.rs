use crate::{
    components::{Player, PlayerCam},
    constants::FOV,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    let player_spawn = Transform::from_xyz(-2.0, 2.0, -5.0);
    let player_height: f32 = 1.75;
    let player_width: f32 = 0.8;
    commands
        .spawn((Player, RigidBody::Dynamic))
        .insert(Collider::cuboid(
            player_width / 2.0,
            player_height / 2.0,
            player_width / 2.0,
        ))
        .insert(SpatialBundle::default())
        .insert(TransformBundle::from(player_spawn))
        .insert(Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        })
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(LockedAxes::ROTATION_LOCKED);

    let cube_width: f32 = 50.0;
    let cube_height: f32 = 0.1;
    let cube_length: f32 = 50.0;
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(cube_width, cube_height, cube_length)),
                material: materials.add(Color::WHITE),
                ..default()
            },
            Collider::cuboid(cube_width / 2.0, cube_height / 2.0, cube_length / 2.0),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));

    let sphere_radius: f32 = 0.5;
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(sphere_radius)),
                material: materials.add(Color::srgb(1.0, 0.0, 1.0)),
                ..default()
            },
            RigidBody::Dynamic,
        ))
        .insert(Collider::ball(sphere_radius))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    /*
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(1.0, 0.0, 1.0)),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    */

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::PI / -4.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        PlayerCam,
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: FOV.to_radians(),
                ..default()
            }),
            ..default()
        },
    ));
}

pub fn setup_physics(mut commands: Commands) {
    /*
    commands
        .spawn(Collider::cuboid(20.0, 0.1, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));
        */
}
