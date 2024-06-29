use crate::{
    shader::{Shader, ShaderError},
    textures::texture::Texture,
    vertex::{ElementBuffer, Vertex, VertexArray, VertexBuffer},
};
use gl::types::GLuint;
use nalgebra_glm as glm;

pub struct Mesh {
    indices: Vec<GLuint>,
    textures: Vec<Texture>,

    vao: VertexArray,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<GLuint>, textures: Vec<Texture>) -> Self {
        let vao = VertexArray::generate();
        let vbo = VertexBuffer::generate();
        let ebo = ElementBuffer::generate();

        vao.bind();
        vbo.bind();
        vbo.buffer_data(&vertices);

        ebo.bind();
        ebo.buffer_data(&indices);

        Vertex::configure_attributes();

        vao.unbind();

        Self {
            indices,
            textures,
            vao,
        }
    }

    pub fn draw_instance(
        &self,
        shader: &Shader,
        model_transform: &glm::Mat4,
        view_transform: &glm::Mat4,
        projection_transform: &glm::Mat4,
    ) -> Result<(), ShaderError> {
        shader.start_using();

        self.vao.bind();

        shader.set_transform("model", model_transform)?;
        shader.set_transform("view", view_transform)?;
        shader.set_transform("projection", projection_transform)?;

        for (i, texture) in self.textures.iter().enumerate() {
            Texture::active(gl::TEXTURE0 + i as u32);
            texture.bind();
        }
        Texture::active(gl::TEXTURE0);
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        self.vao.unbind();

        Ok(())
    }
}
