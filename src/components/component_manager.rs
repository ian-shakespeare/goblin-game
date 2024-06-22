use std::collections::HashMap;
use crate::entity_manager::Entity;
use super::{component_array::ComponentArray, Component, ComponentError, ComponentKind};

pub struct ComponentManager {
    component_arrays: HashMap<ComponentKind, ComponentArray<Component>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_arrays: HashMap::new(),
        }
    }

    pub fn register_component(&mut self, kind: ComponentKind) {
        self.component_arrays.insert(kind, ComponentArray::new());
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) -> Result<(), ComponentError> {
        match self.component_arrays.get_mut(&component.into()) {
            Some(component_array) => {
                component_array.add_entity(entity, component)?;
                Ok(())
            },
            None => Err(ComponentError::UnregisteredComponent(component.into())),
        }
    }

    pub fn get_component(&mut self, entity: Entity, kind: ComponentKind) -> Option<Component> {
        self.component_arrays.get_mut(&kind)?.get_entity(entity)
    }

    pub fn remove_component(&mut self, entity: Entity, kind: ComponentKind) -> Result<(), ComponentError> {
        self.component_arrays.get_mut(&kind).unwrap().remove_entity(entity)?;

        Ok(())
    }
}
