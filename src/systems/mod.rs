use crate::{components::ComponentError, entities::EntityError, shader::ShaderError};

pub mod physics_system;
pub mod render_system;

#[derive(Debug)]
pub enum SystemError {
    EntityError(EntityError),
    ComponentError(ComponentError),
    DrawError(ShaderError),
    LockError,
}

impl From<EntityError> for SystemError {
    fn from(value: EntityError) -> Self {
        SystemError::EntityError(value)
    }
}

impl From<ComponentError> for SystemError {
    fn from(value: ComponentError) -> Self {
        SystemError::ComponentError(value)
    }
}

impl From<ShaderError> for SystemError {
    fn from(value: ShaderError) -> Self {
        SystemError::DrawError(value)
    }
}
