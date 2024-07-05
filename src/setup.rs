use crate::{
    components::{Force, Mass, Player, PlayerCam, Velocity},
    constants::FOV,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_spawn = Transform::from_xyz(-2.0, 2.0, 0.0);
    commands.spawn((
        Player,
        player_spawn,
        Force::default(),
        Velocity::default(),
        Mass(100.0),
    ));

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

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
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
    commands
        .spawn(Collider::cuboid(20.0, 0.1, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
