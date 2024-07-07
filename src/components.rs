use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerCam;

#[derive(Default, Resource, Deref, DerefMut, Debug)]
pub struct MovementInput(Vec3);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);
