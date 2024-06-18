use crate::{shader::{Shader, ShaderError}, vertex::{Vertex, VertexArray, VertexBuffer}};
use nalgebra_glm as glm;

pub struct Triangle<'a> {
    shader: &'a Shader,
    vertices: Vec<Vertex>,
    vao: VertexArray,
    _vbo: VertexBuffer,
}

impl<'a> Triangle<'a> {
    pub fn new(shader: &'a Shader, vertices: Vec<Vertex>) -> Self {
        let vbo = VertexBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = VertexArray::new();
        vao.bind();
        vbo.bind();
        Vertex::configure_attributes();
        vbo.unbind();
        vao.unbind();

        Self {
            vertices,
            shader,
            vao,
            _vbo: vbo,
        }
    }

    pub fn draw(&self, transform: glm::Mat4) -> Result<(), ShaderError> {
        self.shader.start_using();
        self.vao.bind();
        self.shader.set_transform("model", &transform)?;

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }

        Ok(())
    }
}
