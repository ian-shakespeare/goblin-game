use super::SystemError;
use crate::{ecs::ECS, entities::Entity};
use std::{collections::HashSet, sync::Mutex};

pub struct PhysicsSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
}

impl<'a> PhysicsSystem<'a> {
    pub fn init(ecs: &'a Mutex<ECS>) -> Result<Self, SystemError> {
        let entities = HashSet::new();

        Ok(Self { ecs, entities })
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn update(&mut self) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().map_err(|_| SystemError::LockError)?;

        Ok(())
    }
}
