use crate::shader::ShaderError;

pub mod controller_system;
pub mod physics_system;
pub mod render_system;

#[derive(Debug)]
pub enum SystemError {
    EntityError,
    ComponentError,
    DrawError(ShaderError),
    LockError,
    RequestedQuit,
}

impl From<ShaderError> for SystemError {
    fn from(value: ShaderError) -> Self {
        SystemError::DrawError(value)
    }
}

pub trait System {
    fn update(&mut self) -> Result<(), SystemError>;
}
