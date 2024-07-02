use nalgebra_glm::{Vec3, Vec4};

#[derive(Clone, Copy)]
pub struct Transform {
    position: Vec3,
    rotation: Option<Vec4>,
    scale: Option<Vec3>,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Option<Vec4>, scale: Option<Vec3>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn translate(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Option<Vec4> {
        self.rotation
    }

    pub fn scale(&self) -> Option<Vec3> {
        self.scale
    }
}
