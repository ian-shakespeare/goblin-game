use crate::{mesh::Mesh, textures::texture::Texture, vertex::Vertex};
use gl::types::GLuint;

const CUBE_VERTICES: [(f32, f32, f32); 24] = [
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

const CUBE_TEXTURE_COORDS: [(f32, f32); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

const CUBE_NORMALS: [(f32, f32, f32); 6] = [
    (0.0, 0.0, 1.0),
    (0.0, 0.0, -1.0),
    (1.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    (0.0, 1.0, 0.0),
    (0.0, -1.0, 0.0),
];

const CUBE_INDICES: [GLuint; 36] = [
    0, 1, 3, 3, 2, 1, // Up face
    4, 5, 7, 7, 6, 5, // Down face
    8, 9, 11, 11, 10, 9, // North face
    12, 13, 15, 15, 14, 13, // South face
    16, 17, 19, 19, 18, 17, // East face
    20, 21, 23, 23, 22, 21, // West face
];

pub fn get_cube_mesh(textures: Vec<Texture>) -> Mesh {
    let vertices: Vec<Vertex> = CUBE_VERTICES
        .iter()
        .enumerate()
        .map(|(i, vertex)| {
            let tex_coord = CUBE_TEXTURE_COORDS[i % CUBE_TEXTURE_COORDS.len()];
            let normal = CUBE_NORMALS[i / 4];

            (*vertex, normal, tex_coord).into()
        })
        .collect();

    Mesh::new(vertices, CUBE_INDICES.to_vec(), textures)
}
