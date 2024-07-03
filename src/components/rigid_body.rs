use nalgebra_glm::Vec3;

pub struct RigidBody {
    force: Vec3,
    velocity: Vec3,
    height: f32,
    radius: f32,
}

impl RigidBody {
    pub fn new(height: f32, radius: f32) -> Self {
        Self {
            force: Vec3::zeros(),
            velocity: Vec3::zeros(),
            height,
            radius,
        }
    }

    pub fn default() -> Self {
        Self {
            force: Vec3::zeros(),
            velocity: Vec3::zeros(),
            height: 1.0,
            radius: 1.0,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

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

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
