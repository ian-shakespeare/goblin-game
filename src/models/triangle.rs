use crate::{mesh::Mesh, textures::texture::Texture, vertex::Vertex};
use gl::types::GLuint;

const VERTICES: [(f32, f32, f32); 3] = [(-1.0, 0.0, -1.0), (0.0, 0.0, -1.0), (0.0, 0.0, 0.0)];

const TEXTURE_COORDS: [(f32, f32); 3] = [(0.0, 0.0), (0.5, 1.0), (1.0, 0.0)];

const NORMALS: [(f32, f32, f32); 1] = [(0.0, 1.0, 0.0)];

const INDICES: [GLuint; 3] = [0, 1, 2];

pub fn get_triangle_mesh(textures: Vec<Texture>) -> Mesh {
    let vertices: Vec<Vertex> = VERTICES
        .iter()
        .enumerate()
        .map(|(i, vertex)| {
            let tex_coord = TEXTURE_COORDS[i % TEXTURE_COORDS.len()];
            let normal = NORMALS[0];

            (*vertex, normal, tex_coord).into()
        })
        .collect();

    Mesh::new(vertices, INDICES.to_vec(), textures)
}
