use model::ModelComponent;
use texture::TextureComponent;
use transform::TransformComponent;

pub mod component_array;
pub mod component_manager;
pub mod model;
pub mod texture;
pub mod transform;

#[derive(Debug)]
pub enum ComponentError {
    UnregisteredComponent(&'static str),
    EntityWithComponentAlreadyExists,
    NoCorrespondingComponent,
    CannotFreeLastComponent,
    MissingComponent(&'static str),
}

#[derive(Clone, Copy)]
pub enum Component {
    Transform(TransformComponent),
    Model(ModelComponent),
    Texture(TextureComponent),
}

impl From<Component> for &'static str {
    fn from(value: Component) -> Self {
        match value {
            Component::Transform(_) => "Transform",
            Component::Model(_) => "Model",
            Component::Texture(_) => "Texture",
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum ComponentKind {
    Transform,
    Model,
    Texture,
}

impl From<Component> for ComponentKind {
    fn from(value: Component) -> Self {
        match value {
            Component::Transform(_) => ComponentKind::Transform,
            Component::Model(_) => ComponentKind::Model,
            Component::Texture(_) => ComponentKind::Texture,
        }
    }
}
