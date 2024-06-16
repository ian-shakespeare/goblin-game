use std::f32::consts::PI;
use gl::types::{GLint, GLvoid};
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

impl Vertex {
    pub fn new(position: glm::Vec3, texture_coords: glm::Vec2) -> Self {
        Self {
            position,
            texture_coords,
        }
    }

    pub fn configure_attributes() {
        unsafe {
            // position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            // texture
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                std::mem::size_of::<glm::Vec3>() as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);
        }
    }
}
