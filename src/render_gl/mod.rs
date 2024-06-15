use vectors::{Vec2, Vec3};

pub mod shader;
pub mod vectors;

fn create_empty_buffer(len: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    buffer
}

#[repr(C, packed)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub texture_coords: Vec2,
}
