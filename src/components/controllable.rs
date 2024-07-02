use crate::{
    constants::{MOUSE_SENSITIVITY, WORLD_UP},
    utils::degree_to_radian,
};
use nalgebra_glm::{self as glm, Vec3};

pub struct Controllable {
    forward_motion: f32,
    horizontal_motion: f32,
    yaw: f32,
    pitch: f32,
    front: Vec3,
}

impl Controllable {
    pub fn new() -> Self {
        Self {
            forward_motion: 0.0,
            horizontal_motion: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            front: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn calculate_facing(&mut self) {
        let x = degree_to_radian(self.yaw).cos() * degree_to_radian(self.pitch).cos();
        let y = degree_to_radian(self.pitch).sin();
        let z = degree_to_radian(self.yaw).sin() * degree_to_radian(self.pitch).cos();
        let front = Vec3::new(x, y, z);
        // TODO: Determine if this is necessary.
        let _right = glm::normalize(&glm::cross::<f32, glm::U3>(
            &front,
            &Vec3::new(0.0, 1.0, 0.0),
        ));

        self.front = glm::normalize(&front);
    }

    pub fn facing(&self) -> Vec3 {
        self.front
    }

    pub fn perpendicular(&self) -> Vec3 {
        let (x, y, z) = WORLD_UP;
        let up = Vec3::new(x, y, z);
        glm::normalize(&self.front.cross(&up))
    }

    pub fn rotate(&mut self, x: f32, y: f32) {
        self.yaw += MOUSE_SENSITIVITY * x;
        self.pitch = (self.pitch + MOUSE_SENSITIVITY * y).clamp(-89.9, 89.9);
        self.calculate_facing();
    }

    pub fn apply_motion(&mut self, forward_motion: f32, horizontal_motion: f32) {
        self.forward_motion += forward_motion;
        self.horizontal_motion += horizontal_motion;
    }

    pub fn forward_motion(&self) -> f32 {
        self.forward_motion
    }

    pub fn horizontal_motion(&self) -> f32 {
        self.horizontal_motion
    }
}
