use crate::{
    shader::{Shader, ShaderError},
    textures::texture::Texture,
    vertex::{ElementBuffer, Vertex, VertexArray, VertexBuffer},
};
use gl::types::GLuint;
use nalgebra_glm::Mat4;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<GLuint>,
    textures: Vec<Texture>,

    vao: VertexArray,
}

impl<'a> Mesh {
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
            vertices,
            indices,
            textures,
            vao,
        }
    }

    pub fn draw_instance(
        &self,
        shader: &Shader,
        model_transform: &Mat4,
        view_transform: &Mat4,
        projection_transform: &Mat4,
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

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<GLuint> {
        &self.indices
    }

    // WARN: Potential slow down for walking over a vec of references.
    pub fn indexed_vertices(&'a self) -> impl Iterator<Item = &'a Vertex> {
        self.indices
            .iter()
            .map(|index| &self.vertices[*index as usize])
    }
}
