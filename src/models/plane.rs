use crate::{mesh::Mesh, textures::texture::Texture, utils::tuple_to_vec, vertex::Vertex};
use gl::types::GLuint;
use nalgebra_glm::Vec3;

const PLANE_VERTICES: [(f32, f32, f32); 4] = [
    (0.5, 0.5, 0.5),
    (0.5, 0.5, -0.5),
    (-0.5, 0.5, -0.5),
    (-0.5, 0.5, 0.5),
];

const PLANE_TEXTURE_COORDS: [(f32, f32); 4] = [(0.0, 0.0), (0.0, -1.0), (-1.0, -1.0), (-1.0, 0.0)];

const PLANE_NORMAL: (f32, f32, f32) = (0.0, 1.0, 0.0);

const PLANE_INDICES: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

pub struct Plane;

impl Plane {
    pub fn get_vertex_data() -> impl Iterator<Item = ((f32, f32, f32), (f32, f32, f32), (f32, f32))>
    {
        PLANE_VERTICES.iter().enumerate().map(|(i, vertex)| {
            let tex_coord = PLANE_TEXTURE_COORDS[i % PLANE_TEXTURE_COORDS.len()];
            let normal = PLANE_NORMAL;

            (*vertex, normal, tex_coord)
        })
    }

    pub fn get_mesh(textures: Vec<Texture>) -> Mesh {
        let vertices: Vec<Vertex> = Self::get_vertex_data().map(|data| data.into()).collect();

        Mesh::new(vertices, PLANE_INDICES.to_vec(), textures)
    }

    pub fn get_indexed_vertices() -> Vec<(Vec3, Vec3)> {
        PLANE_INDICES
            .iter()
            .map(|index| {
                (
                    tuple_to_vec(PLANE_VERTICES[*index as usize]),
                    tuple_to_vec(PLANE_NORMAL),
                )
            })
            .collect()
    }
}
