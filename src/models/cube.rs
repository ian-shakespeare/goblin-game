use crate::{mesh::Mesh, textures::texture::Texture, utils::tuple_to_vec, vertex::Vertex};
use gl::types::GLuint;
use nalgebra_glm::Vec3;

pub const CUBE_VERTICES: [(f32, f32, f32); 24] = [
    // Up face
    (-0.5, 0.5, -0.5),
    (0.5, 0.5, -0.5),
    (0.5, 0.5, 0.5),
    (-0.5, 0.5, 0.5),
    // Down face
    (-0.5, -0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, -0.5, 0.5),
    (-0.5, -0.5, 0.5),
    // North face
    (0.5, -0.5, -0.5),
    (0.5, 0.5, -0.5),
    (0.5, 0.5, 0.5),
    (0.5, -0.5, 0.5),
    // South face
    (-0.5, -0.5, -0.5),
    (-0.5, 0.5, -0.5),
    (-0.5, 0.5, 0.5),
    (-0.5, -0.5, 0.5),
    // East face
    (-0.5, -0.5, 0.5),
    (0.5, -0.5, 0.5),
    (0.5, 0.5, 0.5),
    (-0.5, 0.5, 0.5),
    // West face
    (-0.5, -0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, 0.5, -0.5),
    (-0.5, 0.5, -0.5),
];

const TEXTURE_COORDS: [(f32, f32); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

pub const CUBE_NORMALS: [(f32, f32, f32); 6] = [
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

pub struct Cube;

impl Cube {
    pub fn get_vertex_data() -> impl Iterator<Item = ((f32, f32, f32), (f32, f32, f32), (f32, f32))>
    {
        CUBE_VERTICES.iter().enumerate().map(|(i, vertex)| {
            let tex_coord = TEXTURE_COORDS[i % TEXTURE_COORDS.len()];
            let normal = CUBE_NORMALS[i / 4];

            (*vertex, normal, tex_coord)
        })
    }

    pub fn get_mesh(textures: Vec<Texture>) -> Mesh {
        let vertices: Vec<Vertex> = Self::get_vertex_data().map(|data| data.into()).collect();

        Mesh::new(vertices, INDICES.to_vec(), textures)
    }

    pub fn get_indexed_vertices() -> Vec<(Vec3, Vec3)> {
        INDICES
            .iter()
            .map(|index| {
                (
                    tuple_to_vec(CUBE_VERTICES[*index as usize]),
                    tuple_to_vec(CUBE_NORMALS[*index as usize / 4]),
                )
            })
            .collect()
    }
}
