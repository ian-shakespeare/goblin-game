use crate::{mesh::Mesh, textures::texture::Texture, vertex::Vertex};
use gl::types::GLuint;

const VERTICES: [(f32, f32, f32); 24] = [
    // Up face
    (-1.0, 0.0, -1.0),
    (0.0, 0.0, -1.0),
    (0.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    // Down face
    (-1.0, -1.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, -1.0, 0.0),
    (-1.0, -1.0, 0.0),
    // North face
    (0.0, -1.0, -1.0),
    (0.0, 0.0, -1.0),
    (0.0, 0.0, 0.0),
    (0.0, -1.0, 0.0),
    // South face
    (-1.0, -1.0, -1.0),
    (-1.0, 0.0, -1.0),
    (-1.0, 0.0, 0.0),
    (-1.0, -1.0, 0.0),
    // East face
    (-1.0, -1.0, 0.0),
    (0.0, -1.0, 0.0),
    (0.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    // West face
    (-1.0, -1.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, 0.0, -1.0),
    (-1.0, 0.0, -1.0),
];

const TEXTURE_COORDS: [(f32, f32); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

const NORMALS: [(f32, f32, f32); 6] = [
    (0.0, 0.0, 1.0),
    (0.0, 0.0, -1.0),
    (1.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    (0.0, 1.0, 0.0),
    (0.0, -1.0, 0.0),
];

const INDICES: [GLuint; 36] = [
    3, 1, 0, 3, 2, 1, // Up face
    4, 5, 7, 5, 6, 7, // Down face
    8, 9, 11, 9, 10, 11, // North face
    15, 13, 12, 15, 14, 13, // South face
    16, 17, 19, 17, 18, 19, // East face
    23, 21, 20, 23, 22, 21, // West face
];

pub fn get_cube_mesh(textures: Vec<Texture>) -> Mesh {
    let vertices: Vec<Vertex> = VERTICES
        .iter()
        .enumerate()
        .map(|(i, vertex)| {
            let tex_coord = TEXTURE_COORDS[i % TEXTURE_COORDS.len()];
            let normal = NORMALS[i / 4];

            (*vertex, normal, tex_coord).into()
        })
        .collect();

    Mesh::new(vertices, INDICES.to_vec(), textures)
}
