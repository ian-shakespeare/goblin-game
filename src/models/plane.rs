use crate::{mesh::Mesh, textures::texture::Texture, vertex::Vertex};
use gl::types::GLuint;

const PLANE_VERTICES: [(f32, f32, f32); 4] = [
    (0.0, 0.0, 0.0),
    (0.0, 0.0, -1.0),
    (-1.0, 0.0, -1.0),
    (-1.0, 0.0, 0.0),
];

const PLANE_TEXTURE_COORDS: [(f32, f32); 4] = [(0.0, 0.0), (0.0, -1.0), (-1.0, -1.0), (-1.0, 0.0)];

const PLANE_NORMALS: [(f32, f32, f32); 1] = [(0.0, 1.0, 0.0)];

const PLANE_INDICES: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

pub fn get_plane_mesh(textures: Vec<Texture>) -> Mesh {
    let vertices: Vec<Vertex> = PLANE_VERTICES
        .iter()
        .enumerate()
        .map(|(i, vertex)| {
            let tex_coord = PLANE_TEXTURE_COORDS[i % PLANE_TEXTURE_COORDS.len()];
            let normal = PLANE_NORMALS[0];

            (*vertex, normal, tex_coord).into()
        })
        .collect();

    Mesh::new(vertices, PLANE_INDICES.to_vec(), textures)
}
