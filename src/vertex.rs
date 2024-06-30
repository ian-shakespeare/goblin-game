use gl::{
    self,
    types::{GLint, GLsizeiptr, GLuint, GLvoid},
};
use nalgebra_glm::{Vec2, Vec3};

#[repr(C, packed)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texture_coordinates: Vec2,
}

impl From<((f32, f32, f32), (f32, f32, f32), (f32, f32))> for Vertex {
    fn from(value: ((f32, f32, f32), (f32, f32, f32), (f32, f32))) -> Self {
        let (position, normal, texture_coords) = value;
        let (pos_x, pos_y, pos_z) = position;
        let (norm_x, norm_y, norm_z) = normal;
        let (tex_x, tex_y) = texture_coords;

        Self {
            position: Vec3::new(pos_x, pos_y, pos_z),
            normal: Vec3::new(norm_x, norm_y, norm_z),
            texture_coordinates: Vec2::new(tex_x, tex_y),
        }
    }
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texture_coordinates: Vec2) -> Self {
        Self {
            position,
            normal,
            texture_coordinates,
        }
    }

    pub fn configure_attributes() {
        unsafe {
            // Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                std::ptr::null(),
            );

            // Normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                std::mem::size_of::<Vec3>() as *const GLvoid,
            );

            // Texture Coordinates
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLint,
                (2 * std::mem::size_of::<Vec3>()) as *const GLvoid,
            );
        }
    }
}

pub struct VertexArray {
    vao: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl VertexArray {
    pub fn generate() -> Self {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        Self { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct VertexBuffer {
    vbo: GLuint,
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl VertexBuffer {
    pub fn generate() -> Self {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Self { vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

pub struct ElementBuffer {
    ebo: GLuint,
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

impl ElementBuffer {
    pub fn generate() -> Self {
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
        }

        Self { ebo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn buffer_data(&self, indices: &[GLuint]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}
