use gl::types::{GLint, GLuint, GLvoid};

pub trait VertexPointer {
    unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize);
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec3 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(other: (f32, f32, f32)) -> Self {
        let (d0, d1, d2) = other;
        Self {
            d0,
            d1,
            d2,
        }
    }
}

impl VertexPointer for Vec3 {
    unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as GLuint);
        gl::VertexAttribPointer(
            location as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as GLint,
            offset as *const GLvoid,
        );
    }
}

impl Vec3 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> Self {
        Self {
            d0,
            d1,
            d2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec4 {
    data: u32,
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        let (x, y, z, w) = other;
        Self::new(x, y, z, w)
    }
}

impl VertexPointer for Vec4 {
    unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as GLuint);
        gl::VertexAttribPointer(
            location as GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as GLint,
            offset as *const GLvoid,
        );
    }
}

impl Vec4 {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let x = (x.clamp(0.0, 1.0) * 1023.0).round() as u32;
        let y = (y.clamp(0.0, 1.0) * 1023.0).round() as u32;
        let z = (z.clamp(0.0, 1.0) * 1023.0).round() as u32;
        let w = (w.clamp(0.0, 1.0) * 3.0).round() as u32;

        let mut data: u32 = 0;
        data |= w << 30;
        data |= z << 20;
        data |= y << 10;
        data |= x;

        Self {
            data,
        }
    }
}
