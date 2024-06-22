use nalgebra_glm as glm;

use crate::{shader::{Shader, ShaderError}, vertex::{Vertex, VertexArray, VertexBuffer}};

pub struct Model<'a> {
    shader: &'a Shader,
    vertex_count: usize,
    vao: VertexArray,
    _vbo: VertexBuffer,
}

impl<'a> Model<'a> {
    pub fn from_vertices(shader: &'a Shader, vertices: Vec<Vertex>) -> Self {
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
            vertex_count: vertices.len(),
            shader,
            vao,
            _vbo: vbo,
        }
    }

    pub fn shader(&self) -> &Shader {
        self.shader
    }

    pub fn draw_instance(
        &self,
        model_transform: &glm::Mat4,
        view_transform: &glm::Mat4,
        projection_transform: &glm::Mat4
    ) -> Result<(), ShaderError> {
        self.shader.start_using();

        self.vao.bind();
        self.shader.set_transform("model", model_transform)?;
        self.shader.set_transform("view", view_transform)?;
        self.shader.set_transform("projection", projection_transform)?;

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
        }

        Ok(())
    }
}
