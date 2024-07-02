use nalgebra_glm::Vec3;

pub struct CameraFollowable {
    camera_relative_position: Vec3,
    is_being_followed: bool,
}

impl<'a> CameraFollowable {
    pub fn new(is_being_followed: bool, camera_relative_position: Vec3) -> Self {
        Self {
            is_being_followed,
            camera_relative_position,
        }
    }

    pub fn followed(&self) -> bool {
        self.is_being_followed
    }

    pub fn camera_relative_position(&self) -> Vec3 {
        self.camera_relative_position
    }
}
