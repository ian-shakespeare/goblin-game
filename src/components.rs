use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerCam;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec3::ZERO)
    }
}

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

#[derive(Component)]
pub struct Force(pub Vec3);

impl Force {
    pub fn apply_force(&mut self, force: Vec3) {
        self.0 += force
    }

    pub fn reset(&mut self) {
        self.0 = Vec3::ZERO;
    }
}

impl Default for Force {
    fn default() -> Self {
        Force(Vec3::ZERO)
    }
}
