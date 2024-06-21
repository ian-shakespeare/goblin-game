use nalgebra_glm as glm;
use crate::{shader::Shader, vertex::Vertex};
use super::{polygon_renderer::PolygonRenderer, Polygon};

const CUBE_VERTICES: [(f32, f32, f32);36] = [
    (-1.0, -1.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, 0.0, -1.0),
    (0.0, 0.0, -1.0),
    (-1.0, 0.0, -1.0),
    (-1.0, -1.0, -1.0),

    (-1.0, -1.0, 0.0),
    (0.0, -1.0, 0.0),
    (0.0, 0.0, 0.0),
    (0.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    (-1.0, -1.0, 0.0),

    (-1.0, 0.0, 0.0),
    (-1.0, 0.0, -1.0),
    (-1.0, -1.0, -1.0),
    (-1.0, -1.0, -1.0),
    (-1.0, -1.0, 0.0),
    (-1.0, 0.0, 0.0),

    (0.0, 0.0, 0.0),
    (0.0, 0.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, -1.0, 0.0),
    (0.0, 0.0, 0.0),

    (-1.0, -1.0, -1.0),
    (0.0, -1.0, -1.0),
    (0.0, -1.0, 0.0),
    (0.0, -1.0, 0.0),
    (-1.0, -1.0, 0.0),
    (-1.0, -1.0, -1.0),

    (-1.0, 0.0, -1.0),
    (0.0, 0.0, -1.0),
    (0.0, 0.0, 0.0),
    (0.0, 0.0, 0.0),
    (-1.0, 0.0, 0.0),
    (-1.0, 0.0, -1.0),
];

const DEFAULT_TEX_COORDS: [(f32, f32);6] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (1.0, 1.0),
    (0.0, 1.0),
    (0.0, 0.0),
];

pub struct Cube;

impl Polygon for Cube {
    fn new(shader: &Shader) -> PolygonRenderer {
        let mut vertices: Vec<Vertex> = Vec::new();
        for (i, (x, y, z)) in CUBE_VERTICES.iter().enumerate() {
            let (tex_x, tex_y) = DEFAULT_TEX_COORDS[i % 6];
            vertices.push(
                Vertex::new(glm::Vec3::new(*x, *y, *z), glm::Vec2::new(tex_x, tex_y)),
            );
        }

        PolygonRenderer::create_polygon(shader, vertices)
    }

    fn with_scaled_tex(shader: &Shader, scale_tex_x: f32, scale_tex_y: f32) -> PolygonRenderer {
        let mut vertices: Vec<Vertex> = Vec::new();
        for (i, (x, y, z)) in CUBE_VERTICES.iter().enumerate() {
            let (tex_x, tex_y) = DEFAULT_TEX_COORDS[i % 6];
            vertices.push(
                Vertex::new(glm::Vec3::new(*x, *y, *z), glm::Vec2::new(tex_x * scale_tex_x, tex_y * scale_tex_y)),
            );
        }

        PolygonRenderer::create_polygon(shader, vertices)
    }
}
