use nalgebra_glm as glm;
use crate::{shader::Shader, vertex::Vertex};

use super::{polygon_renderer::PolygonRenderer, Polygon};

const LINE_VERTICES: [(f32, f32, f32); 2] = [
    (0.0, 0.0, 0.0),
    (1.0, 0.0, 0.0),
];

const DEFAULT_TEX_COORDS: [(f32, f32);6] = [
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 1.0),
];

pub struct Line;

impl Polygon for Line {
    fn new(shader: &Shader) -> PolygonRenderer {
        let mut vertices = Vec::new();
        for (i, (x, y, z)) in LINE_VERTICES.iter().enumerate() {
            let (tex_x, tex_y) = DEFAULT_TEX_COORDS[i % 6];
            vertices.push(
                Vertex::new(glm::Vec3::new(*x, *y, *z), glm::Vec2::new(tex_x, tex_y)),
            );
        }

        PolygonRenderer::create_polygon(shader, vertices)
    }

    // TODO(ian) figure out colors without textures
    fn with_scaled_tex(shader: &Shader, _: f32, _: f32) -> PolygonRenderer {
        let mut vertices = Vec::new();
        for (i, (x, y, z)) in LINE_VERTICES.iter().enumerate() {
            let (tex_x, tex_y) = DEFAULT_TEX_COORDS[i % 6];
            vertices.push(
                Vertex::new(glm::Vec3::new(*x, *y, *z), glm::Vec2::new(tex_x, tex_y)),
            );
        }

        PolygonRenderer::create_polygon(shader, vertices)
    }
}
