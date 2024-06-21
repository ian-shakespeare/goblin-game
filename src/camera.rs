use nalgebra_glm as glm;
use crate::utils::degree_to_radian;

const DEFAULT_YAW: f32 = 0.0;
const DEFAULT_PITCH: f32 = 0.0;
const DEFAULT_FOV: f32 = 45.0;
const DEFAULT_POSITION: (f32, f32, f32) = (0.0, 1.0, 0.0);
const DEFAULT_WORLD_UP: (f32, f32, f32) = (0.0, 1.0, 0.0);
const DEFAULT_FRONT: (f32, f32, f32) = (0.0, 0.0, -1.0);
const DEFAULT_SENSITIVITY: f32 = 0.1;
const DEFAULT_SPEED: f32 = 0.1;

pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    yaw: f32,
    pitch: f32,
    fov: f32,
    speed: f32,
}

impl Camera {
    /// Create a camera with default values
    pub fn new() -> Self {
        let yaw = DEFAULT_YAW;
        let pitch = DEFAULT_PITCH;
        let position = glm::Vec3::new(DEFAULT_POSITION.0, DEFAULT_POSITION.1, DEFAULT_POSITION.2);
        let world_up = glm::Vec3::new(DEFAULT_WORLD_UP.0, DEFAULT_WORLD_UP.1, DEFAULT_WORLD_UP.2);
        let front = glm::Vec3::new(DEFAULT_FRONT.0, DEFAULT_FRONT.1, DEFAULT_FRONT.2);
        let right = glm::Vec3::identity();
        let up = glm::Vec3::identity();
        let fov = DEFAULT_FOV;
        let speed = DEFAULT_SPEED;
        let mut camera = Self {
            position,
            front,
            right,
            world_up,
            up,
            yaw,
            pitch,
            fov,
            speed,
        };
        camera.update_vectors();

        camera
    }

    pub fn from_vectors(
        position: glm::Vec3,
        world_up: glm::Vec3,
        yaw: f32,
        pitch: f32,
        fov: Option<f32>,
        speed: Option<f32>,
    ) -> Self {
        let front = glm::Vec3::new(DEFAULT_FRONT.0, DEFAULT_FRONT.1, DEFAULT_FRONT.2);
        let right = glm::Vec3::identity();
        let up = glm::Vec3::identity();
        let fov = fov.unwrap_or(DEFAULT_FOV);
        let speed = speed.unwrap_or(DEFAULT_SPEED);
        let mut camera = Self {
            position,
            front,
            right,
            world_up,
            up,
            yaw,
            pitch,
            fov,
            speed,
        };
        camera.update_vectors();

        camera
    }

    fn update_vectors(&mut self) {
        let mut front = glm::Vec3::identity();
        front.x = degree_to_radian(self.yaw).cos() * degree_to_radian(self.pitch).cos();
        front.y = degree_to_radian(self.pitch).sin();
        front.z = degree_to_radian(self.yaw).sin() * degree_to_radian(self.pitch).cos();

        self.front = glm::normalize(&front);
        self.right = glm::normalize(&glm::cross::<f32, glm::U3>(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross::<f32, glm::U3>(&self.right, &self.front));
    }

    pub fn translate(&mut self, new_position: glm::Vec3) {
        self.position += new_position;
    }

    pub fn rotate(&mut self, x: f32, y: f32, sensitivity: Option<f32>) {
        self.yaw += x * sensitivity.unwrap_or(DEFAULT_SENSITIVITY);
        self.pitch = (self.pitch + y * sensitivity.unwrap_or(DEFAULT_SENSITIVITY)).clamp(-89.0, 89.0);

        self.update_vectors();
    }

    pub fn zoom(&mut self, degrees: f32) {
        self.fov = (self.fov + degrees).clamp(1.0, 90.0);
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn front(&self) -> glm::Vec3 {
        self.front
    }

    pub fn up(&self) -> glm::Vec3 {
        self.up
    }

    /// Camera fov in radians
    pub fn fov(&self) -> f32 {
        degree_to_radian(self.fov)
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}
