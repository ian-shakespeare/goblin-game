use gl::{self, types::{GLint, GLsizeiptr, GLuint, GLvoid}};
use nalgebra_glm as glm;

#[repr(C, packed)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub texture_coords: glm::Vec2,
}

impl From<((f32, f32, f32), (f32, f32))> for Vertex {
    fn from(value: ((f32, f32, f32), (f32, f32))) -> Self {
        let (position, texture_coords) = value;
        let (pos_x, pos_y, pos_z) = position;
        let (tex_x, tex_y) = texture_coords;

        Self {
            position: glm::Vec3::new(pos_x, pos_y, pos_z),
            texture_coords: glm::Vec2::new(tex_x, tex_y),
        }
    }
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
    pub fn new() -> Self {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        Self {
            vao,
        }
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
    pub fn new() -> Self {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Self {
            vbo,
        }
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

    pub fn static_draw_data<T>(&self, data: &[T]) {
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
    pub fn new() -> Self {
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
        }

        Self {
            ebo,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn static_draw_indices<T>(&self, indices: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
           );
        }
    }
}
