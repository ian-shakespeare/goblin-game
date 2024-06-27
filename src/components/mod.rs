use gravity::GravityComponent;
use model::ModelComponent;
use rigid_body::RigidBodyComponent;
use texture::TextureComponent;
use transform::TransformComponent;

pub mod component_array;
pub mod component_manager;
pub mod gravity;
pub mod model;
pub mod rigid_body;
pub mod texture;
pub mod transform;

pub trait Component {
    fn get_kind() -> ComponentKind;

    fn get_value(self) -> ComponentValue;
}

#[derive(Debug)]
pub enum ComponentError {
    UnregisteredComponent(&'static str),
    EntityWithComponentAlreadyExists,
    NoCorrespondingComponent,
    CannotFreeLastComponent,
    MissingComponent(&'static str),
}

#[derive(Clone, Copy)]
pub enum ComponentValue {
    Transform(TransformComponent),
    Model(ModelComponent),
    Texture(TextureComponent),
    Gravity(GravityComponent),
    RigidBody(RigidBodyComponent),
}

impl From<ComponentValue> for &'static str {
    fn from(value: ComponentValue) -> Self {
        match value {
            ComponentValue::Transform(_) => "Transform",
            ComponentValue::Model(_) => "Model",
            ComponentValue::Texture(_) => "Texture",
            ComponentValue::Gravity(_) => "Gravity",
            ComponentValue::RigidBody(_) => "RigidBody",
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ComponentKind {
    Transform,
    Model,
    Texture,
    Gravity,
    RigidBody,
}

impl ComponentKind {
    pub const VALUES: [Self; 5] = [
        Self::Transform,
        Self::Model,
        Self::Texture,
        Self::Gravity,
        Self::RigidBody,
    ];
}

impl From<ComponentValue> for ComponentKind {
    fn from(value: ComponentValue) -> Self {
        match value {
            ComponentValue::Transform(_) => ComponentKind::Transform,
            ComponentValue::Model(_) => ComponentKind::Model,
            ComponentValue::Texture(_) => ComponentKind::Texture,
            ComponentValue::Gravity(_) => ComponentKind::Gravity,
            ComponentValue::RigidBody(_) => ComponentKind::RigidBody,
        }
    }
}
