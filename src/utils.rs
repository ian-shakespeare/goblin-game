use std::f32::consts::PI;
use nalgebra_glm as glm;

pub fn create_empty_buffer(len: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    buffer
}

pub fn degree_to_radian(degree: f32) -> f32 {
    degree * (PI / 180.0)
}

#[repr(C, packed)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub texture_coords: glm::Vec2,
}
