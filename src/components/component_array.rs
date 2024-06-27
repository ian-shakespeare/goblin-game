use crate::entities::{Entity, MAX_ENTITIES};
use std::collections::HashMap;

use super::ComponentError;

pub struct ComponentArray<T> {
    components: [Option<T>; MAX_ENTITIES],
    entity_index_lookup: HashMap<u32, usize>,
    index_entity_lookup: HashMap<usize, u32>,
    count: usize,
}

impl<T> ComponentArray<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        let components = [None; MAX_ENTITIES];
        Self {
            entity_index_lookup: HashMap::new(),
            index_entity_lookup: HashMap::new(),
            count: 0,
            components,
        }
    }

    pub fn add_entity(&mut self, entity: Entity, data: T) -> Result<(), ComponentError> {
        if self.entity_index_lookup.get(&entity).is_some() {
            return Err(ComponentError::EntityWithComponentAlreadyExists);
        }

        let new_index = self.count;
        self.entity_index_lookup.insert(entity, new_index);
        self.index_entity_lookup.insert(new_index, entity);
        self.components[new_index] = Some(data);
        self.count += 1;

        Ok(())
    }

    pub fn get_entity(&self, entity: Entity) -> Option<T> {
        if entity >= MAX_ENTITIES as u32 {
            return None;
        }
        let index = self.entity_index_lookup.get(&entity)?;

        *self.components.get(*index)?
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), ComponentError> {
        // TODO: Handle removing last entity better.

        let index_of_removed = self
            .entity_index_lookup
            .get(&entity)
            .ok_or(ComponentError::NoCorrespondingComponent)?
            .clone();
        let index_of_last = self.count - 1;

        self.components[index_of_removed] = self.components[index_of_last];

        let entity_of_last = self
            .index_entity_lookup
            .get(&index_of_last)
            .ok_or(ComponentError::CannotFreeLastComponent)?;
        self.entity_index_lookup
            .insert(*entity_of_last, index_of_removed);
        self.index_entity_lookup
            .insert(index_of_removed, *entity_of_last);

        self.entity_index_lookup.remove(&entity);
        self.index_entity_lookup.remove(&index_of_last);

        self.count -= 1;

        Ok(())
    }
}
