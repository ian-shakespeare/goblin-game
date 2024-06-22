use nalgebra_glm as glm;
use std::{collections::HashSet, rc::Rc, sync::Mutex};
use crate::{
    components::{
        component_manager::ComponentManager,
        model::ModelComponent,
        texture::TextureComponent,
        transform::TransformComponent,
        Component,
        ComponentError,
        ComponentKind,
    },
    entity_manager::{Entity, EntityError, EntityManager},
    models::model_manager::{ModelId, ModelManager},
    shader::ShaderError,
    textures::texture_manager::{TextureId, TextureManager}, utils::degree_to_radian,
};

#[derive(Debug)]
pub enum RenderSystemError {
    EntityError(EntityError),
    ComponentError(ComponentError),
    DrawError(ShaderError),
}

impl From<EntityError> for RenderSystemError {
    fn from(value: EntityError) -> Self {
        RenderSystemError::EntityError(value)
    }
}

impl From<ComponentError> for RenderSystemError {
    fn from(value: ComponentError) -> Self {
        RenderSystemError::ComponentError(value)
    }
}

impl From<ShaderError> for RenderSystemError {
    fn from(value: ShaderError) -> Self {
        RenderSystemError::DrawError(value)
    }
}

pub struct RenderSystem<'a> {
    component_manager: Rc<Mutex<ComponentManager>>,
    entity_manager: Rc<Mutex<EntityManager>>,
    entities: HashSet<Entity>,
    model_manager: ModelManager<'a>,
    texture_manager: TextureManager,
}

impl<'a> RenderSystem<'a> {
    pub fn init(
        entity_manager: Rc<Mutex<EntityManager>>,
        component_manager: Rc<Mutex<ComponentManager>>,
        model_manager: ModelManager<'a>,
        texture_manager: TextureManager,
    ) -> Result<Self, RenderSystemError> {
        let entities = HashSet::new();

        Ok(Self {
            component_manager,
            entity_manager,
            entities,
            model_manager,
            texture_manager,
        })
    }

    pub fn add_item(
        &mut self,
        position: glm::Vec3,
        rotation: glm::Vec4,
        scale: glm::Vec3,
        model_id: ModelId,
        texture_id: TextureId,
    ) -> Result<(), RenderSystemError> {
        let model = ModelComponent { id: model_id, tex_coords: [glm::Vec2::identity(); 6] };
        let transform = TransformComponent { position, rotation, scale };
        let texture = TextureComponent { id: texture_id };

        let mut entity_manager = self.entity_manager
            .lock()
            .expect("Could not lock entity manager");
        let entity = entity_manager.create_entity()?;

        let mut component_manager = self.component_manager
            .lock()
            .expect("Could not lock component manager");
        component_manager
            .add_component(entity, Component::Model(model))?;
        component_manager
            .add_component(entity, Component::Transform(transform))?;
        component_manager
            .add_component(entity, Component::Texture(texture))?;

        self.entities.insert(entity);

        Ok(())
    }

    pub fn draw(&mut self, view_transform: &glm::Mat4, projection_transform: &glm::Mat4) -> Result<(), RenderSystemError> {
        for entity in &self.entities {
            let mut component_manager = self.component_manager
                .lock()
                .expect("Could not lock mutex");
            let transform_component = component_manager
                .get_component(*entity, ComponentKind::Transform)
                .ok_or(RenderSystemError::ComponentError(ComponentError::UnregisteredComponent("Transform")))?;
            let TransformComponent { position, rotation, scale } = match transform_component {
                Component::Transform(transform) => transform,
                _ => return Err(RenderSystemError::ComponentError(ComponentError::MissingComponent("Transform"))),
            };
            let mut model_transform = glm::Mat4::identity();
            model_transform = glm::translate(&model_transform, &position);
            if rotation[0] >= 0.0 {
                model_transform = glm::rotate(
                    &model_transform,
                    degree_to_radian(rotation[0]),
                    &glm::Vec3::new(rotation[1], rotation[2], rotation[3]),
                );
            }
            model_transform = glm::scale(&model_transform, &scale);

            let texture_component = component_manager
                .get_component(*entity, ComponentKind::Texture)
                .ok_or(RenderSystemError::ComponentError(ComponentError::UnregisteredComponent("Texture")))?;
            let TextureComponent { id } = match texture_component {
                Component::Texture(texture) => texture,
                _ => return Err(RenderSystemError::ComponentError(ComponentError::MissingComponent("Texture"))),
            };
            let texture = self.texture_manager.get_texture(id);
            texture.bind();

            let model_component = component_manager.get_component(*entity, ComponentKind::Model)
                .ok_or(RenderSystemError::ComponentError(ComponentError::UnregisteredComponent("Model")))?;
            let ModelComponent { id, tex_coords: _ } = match model_component {
                Component::Model(model) => model,
                _ => return Err(RenderSystemError::ComponentError(ComponentError::MissingComponent("Model"))),
            };
            let model = self.model_manager.get_model(id);
            model.draw_instance(&model_transform, view_transform, projection_transform)
                .expect("Shader error drawing entity");
        }

        Ok(())
    }
}
