use crate::collider::Hitbox;
use nalgebra_glm::Vec3;

pub struct RigidBody {
    hitbox: Hitbox,
    force: Vec3,
    velocity: Vec3,
}

impl RigidBody {
    pub fn new(hitbox: Hitbox) -> Self {
        Self {
            force: Vec3::zeros(),
            velocity: Vec3::zeros(),
            hitbox,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    // pub fn apply_drag(&mut self, force: Vec3) {}

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    pub fn reset_force(&mut self) {
        self.force = Vec3::zeros();
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn net_force(&self) -> Vec3 {
        self.force
    }
}
