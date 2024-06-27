use super::{signature::Signature, Entity, EntityError, MAX_ENTITIES};
use std::collections::VecDeque;

pub struct EntityManager {
    available_entities: VecDeque<Entity>,
    signatures: [u32; MAX_ENTITIES],
    count: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let mut available_entities = VecDeque::new();
        for i in 0..MAX_ENTITIES {
            available_entities.push_back(i as u32);
        }

        Self {
            count: 0,
            signatures: [0; MAX_ENTITIES],
            available_entities,
        }
    }

    pub fn create_entity(&mut self) -> Result<Entity, EntityError> {
        let entity = self
            .available_entities
            .pop_front()
            .ok_or(EntityError::ExceededMaxEntities)?;
        self.count += 1;

        Ok(entity)
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> Result<(), EntityError> {
        if entity as usize >= MAX_ENTITIES {
            return Err(EntityError::OutOfRange);
        }
        self.delete_signature(entity)?;
        self.available_entities.push_back(entity);

        Ok(())
    }

    pub fn add_signature(
        &mut self,
        entity: Entity,
        signature: Signature,
    ) -> Result<(), EntityError> {
        let entity = entity as usize;
        if entity >= MAX_ENTITIES {
            return Err(EntityError::OutOfRange);
        }
        self.signatures[entity] |= signature.get_value();

        Ok(())
    }

    pub fn remove_signature(
        &mut self,
        entity: Entity,
        signature: Signature,
    ) -> Result<(), EntityError> {
        let entity = entity as usize;
        if entity >= MAX_ENTITIES {
            return Err(EntityError::OutOfRange);
        }
        self.signatures[entity] &= !signature.get_value();

        Ok(())
    }

    pub fn delete_signature(&mut self, entity: Entity) -> Result<(), EntityError> {
        let entity = entity as usize;
        if entity >= MAX_ENTITIES {
            return Err(EntityError::OutOfRange);
        }
        self.signatures[entity] = 0x0000;

        Ok(())
    }
}
