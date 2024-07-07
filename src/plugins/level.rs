use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct Level;

impl Level {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
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
    }
}

impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
    }
}
