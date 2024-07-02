use super::{System, SystemError};
use crate::{
    camera::Camera,
    components::{
        camera_followable::CameraFollowable, controllable::Controllable, mesh::MeshComponent,
        transform::Transform,
    },
    constants::CAMERA_FOV,
    ecs::Ecs,
    mesh_manager::MeshManager,
    shader::Shader,
    utils::create_transform_matrix,
};
use std::sync::Mutex;

pub struct RenderSystem<'a> {
    ecs: &'a Mutex<Ecs>,
    mesh_manager: &'a Mutex<MeshManager>,
    shader: &'a Shader,
}

impl<'a> RenderSystem<'a> {
    pub fn init(
        ecs: &'a Mutex<Ecs>,
        mesh_manager: &'a Mutex<MeshManager>,
        shader: &'a Shader,
    ) -> Self {
        Self {
            ecs,
            mesh_manager,
            shader,
        }
    }
}

impl<'a> System for RenderSystem<'a> {
    fn update(&mut self) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;
        let mesh_manager = self
            .mesh_manager
            .lock()
            .map_err(|_| SystemError::LockError)?;

        let controlled = ecs
            .get_component_vec::<Controllable>()
            .expect("Could not get controlled component");
        let camera_followable = ecs
            .get_component_vec::<CameraFollowable>()
            .expect("Could not get camera followable component");
        let transforms = ecs
            .get_component_vec::<Transform>()
            .expect("Could not get transform component");

        let (camera_control, camera_follow, camera_transform) = controlled
            .iter()
            .zip(camera_followable.iter().zip(transforms.iter()))
            .find_map(|(controlled, (camera_followable, transform))| {
                if let Some(camera_followable) = camera_followable {
                    if camera_followable.followed() {
                        return Some((
                            controlled.as_ref()?,
                            camera_followable,
                            transform.as_ref()?,
                        ));
                    }
                }
                None
            })
            .expect("Missing camera follow");

        let camera_position =
            camera_transform.position() + camera_follow.camera_relative_position();
        let view_transform = Camera::view_transform(&camera_position, &camera_control.facing());
        let projection_transform = Camera::projection_transform(CAMERA_FOV);

        let meshes = ecs
            .get_component_vec::<MeshComponent>()
            .expect("Could not get component vector");

        let union = transforms
            .iter()
            .zip(meshes.iter())
            .filter_map(|(transform, mesh)| Some((transform.as_ref()?, mesh.as_ref()?)));

        for (transform, mesh) in union {
            let MeshComponent { id } = mesh;

            let model_transform = create_transform_matrix(transform);

            let mesh = mesh_manager.get_mesh(*id).expect("Missing mesh");
            mesh.draw_instance(
                self.shader,
                &model_transform,
                &view_transform,
                &projection_transform,
            )
            .expect("Shader error drawing entity");
        }

        Ok(())
    }
}
