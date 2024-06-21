use std::rc::Rc;
use nalgebra_glm as glm;
use crate::{textures::texture::Texture, utils::degree_to_radian, polygons::polygon_renderer::PolygonRenderer};

pub struct Entity<'a> {
    renderer: PolygonRenderer<'a>,
    texture: Rc<Texture>,
    transform: glm::Mat4,
}

impl<'a> Entity<'a> {
    pub fn new(renderer: PolygonRenderer<'a>, texture: Rc<Texture>) -> Self {
        Self {
            transform: glm::Mat4::identity(),
            renderer,
            texture,
        }
    }

    pub fn translate(&mut self, vec: glm::Vec3) {
        self.transform = glm::translate(&self.transform, &vec);
    }

    pub fn rotate(&mut self, angle: f32, axis: glm::Vec3) {
        self.transform = glm::rotate(&self.transform, degree_to_radian(angle), &axis);
    }

    pub fn scale(&mut self, vec: glm::Vec3) {
        self.transform = glm::scale(&self.transform, &vec);
    }

    pub fn draw(&self, view_transform: &glm::Mat4, projection_transform: &glm::Mat4) {
        self.renderer.shader().start_using();
        self.renderer.shader().set_uniform_1i("aTexture", 0).expect("Shader error applying texture uniform");
        self.texture.bind();
        self.renderer.shader().set_transform("view", view_transform).expect("Shader error applying view transform");
        self.renderer.shader().set_transform("projection", projection_transform).expect("Shader error applying projection transform");
        self.renderer.draw_instance(self.transform)
            .expect("Shader error drawing entity");
    }
}
