use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH, WORLD_UP},
    utils::degree_to_radian,
};
use nalgebra_glm::{self as glm, Mat4, Vec3};

pub struct Camera {
    fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self { fov: 45.0 }
    }

    pub fn view_transform(position: &Vec3, front: &Vec3) -> Mat4 {
        let (x, y, z) = WORLD_UP;
        let up = Vec3::new(x, y, z);
        glm::look_at(&position, &(position + front), &up)
    }

    pub fn projection_transform(fov: f32) -> Mat4 {
        glm::perspective::<f32>(
            SCREEN_WIDTH / SCREEN_HEIGHT,
            degree_to_radian(fov),
            0.1,
            100.0,
        )
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn zoom(&mut self, degrees: f32) {
        self.fov = (self.fov + degrees).clamp(1.0, 90.0);
    }
}
