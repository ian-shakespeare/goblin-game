pub mod shader;

#[repr(C, packed)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Vec2 {
    fn from(other: (f32, f32)) -> Self {
        let (x, y) = other;
        Self {
            x,
            y,
        }
    }
}

#[repr(C, packed)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(other: (f32, f32, f32)) -> Self {
        let (x, y, z) = other;
        Self {
            x,
            y,
            z,
        }
    }
}

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
