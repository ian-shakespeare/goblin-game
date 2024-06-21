use polygon_renderer::PolygonRenderer;
use crate::shader::Shader;

pub mod polygon_renderer;
pub mod cube;
pub mod line;
pub mod plane;

pub trait Polygon {
    fn new(shader: &Shader) -> PolygonRenderer;

    fn with_scaled_tex(shader: &Shader, scale_tex_x: f32, scale_tex_y: f32) -> PolygonRenderer;
}
