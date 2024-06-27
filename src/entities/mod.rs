pub mod entity_manager;
pub mod signature;

pub const MAX_ENTITIES: usize = 5000;

pub type Entity = u32;

#[derive(Debug)]
pub enum EntityError {
    ExceededMaxEntities,
    OutOfRange,
}
