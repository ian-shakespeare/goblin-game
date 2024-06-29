use collision::CollisionComponent;
use gravity::GravityComponent;
use mesh::MeshComponent;
use rigid_body::RigidBodyComponent;
use texture::TextureComponent;
use transform::TransformComponent;

pub mod collision;
pub mod component_array;
pub mod component_manager;
pub mod gravity;
pub mod mesh;
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
    OutOfRange,
}

#[derive(Clone, Copy)]
pub enum ComponentValue {
    Transform(TransformComponent),
    Mesh(MeshComponent),
    Texture(TextureComponent),
    Gravity(GravityComponent),
    RigidBody(RigidBodyComponent),
    Collision(CollisionComponent),
}

impl From<ComponentValue> for &'static str {
    fn from(value: ComponentValue) -> Self {
        match value {
            ComponentValue::Transform(_) => "Transform",
            ComponentValue::Mesh(_) => "Mesh",
            ComponentValue::Texture(_) => "Texture",
            ComponentValue::Gravity(_) => "Gravity",
            ComponentValue::RigidBody(_) => "RigidBody",
            ComponentValue::Collision(_) => "Collision",
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ComponentKind {
    Transform,
    Mesh,
    Texture,
    Gravity,
    RigidBody,
    Collision,
}

impl ComponentKind {
    pub const VALUES: [Self; 6] = [
        Self::Transform,
        Self::Mesh,
        Self::Texture,
        Self::Gravity,
        Self::RigidBody,
        Self::Collision,
    ];
}

impl<'a> From<ComponentValue> for ComponentKind {
    fn from(value: ComponentValue) -> Self {
        match value {
            ComponentValue::Transform(_) => ComponentKind::Transform,
            ComponentValue::Mesh(_) => ComponentKind::Mesh,
            ComponentValue::Texture(_) => ComponentKind::Texture,
            ComponentValue::Gravity(_) => ComponentKind::Gravity,
            ComponentValue::RigidBody(_) => ComponentKind::RigidBody,
            ComponentValue::Collision(_) => ComponentKind::Collision,
        }
    }
}
