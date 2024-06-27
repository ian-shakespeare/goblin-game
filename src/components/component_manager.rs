use super::{
    component_array::ComponentArray, Component, ComponentError, ComponentKind, ComponentValue,
};
use crate::entities::Entity;
use std::collections::HashMap;

pub struct ComponentManager {
    component_arrays: HashMap<ComponentKind, ComponentArray<ComponentValue>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_arrays: HashMap::new(),
        }
    }

    pub fn register_component<C: Component>(&mut self) {
        let kind = C::get_kind();
        self.component_arrays.insert(kind, ComponentArray::new());
    }

    pub fn add_component<C: Component>(
        &mut self,
        entity: Entity,
        component: C,
    ) -> Result<(), ComponentError> {
        let component = component.get_value();
        match self.component_arrays.get_mut(&component.into()) {
            Some(component_array) => {
                component_array.add_entity(entity, component)?;
                Ok(())
            }
            None => Err(ComponentError::UnregisteredComponent(component.into())),
        }
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<ComponentValue> {
        let kind = C::get_kind();
        self.component_arrays.get(&kind)?.get_entity(entity)
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Result<(), ComponentError> {
        let kind = C::get_kind();
        self.component_arrays
            .get_mut(&kind)
            .unwrap()
            .remove_entity(entity)?;

        Ok(())
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        for (_, array) in &mut self.component_arrays {
            // Ignore arrays that fail to remove entity.
            let _ = array.remove_entity(entity);
        }
    }
}
