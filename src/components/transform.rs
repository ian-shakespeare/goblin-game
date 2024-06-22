use nalgebra_glm as glm;

#[derive(Clone, Copy, PartialEq)]
pub struct TransformComponent {
    pub position: glm::Vec3,
    pub rotation: glm::Vec4,
    pub scale: glm::Vec3,
}
