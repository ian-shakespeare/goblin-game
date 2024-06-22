use std::collections::VecDeque;

pub const MAX_ENTITIES: usize = 5000;

pub type Entity = u32;

#[derive(Debug)]
pub enum EntityError {
    ExceededMaxEntities,
}

pub struct EntityManager {
    available_entities: VecDeque<Entity>,
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
            available_entities,
        }
    }

    pub fn create_entity(&mut self) -> Result<Entity, EntityError> {
        let entity = self.available_entities
            .pop_front()
            .ok_or(EntityError::ExceededMaxEntities)?;
        self.count += 1;

        Ok(entity)
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        self.available_entities.push_back(entity);
    }
}
