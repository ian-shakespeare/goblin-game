use crate::{
    components::{component_manager::ComponentManager, Component, ComponentValue},
    entities::{entity_manager::EntityManager, signature::Signature, Entity},
};

pub struct ECS {
    component_manager: ComponentManager,
    entity_manager: EntityManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager
            .create_entity()
            .expect("Could not create entity.")
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.component_manager.destroy_entity(entity);
        self.entity_manager
            .destroy_entity(entity)
            .expect("Entity manager could not destroy entity.");
    }

    pub fn register_component<C: Component>(&mut self) {
        self.component_manager.register_component::<C>();
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
        let kind = C::get_kind();
        self.entity_manager
            .add_signature(entity, kind.into())
            .expect("Could not add signature");
        self.component_manager
            .add_component(entity, component)
            .expect("Could not add component to manager.");
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<C> {
        self.component_manager.get_component::<C>(entity)
    }

    pub fn get_all_entities_with_component<C: Component>(&self) -> Vec<Entity> {
        let signature: Signature = C::get_kind().into();
        self.entity_manager
            .get_all_entities_with_signature(signature)
    }

    pub fn set_component(&mut self, entity: Entity, component: ComponentValue) {
        self.component_manager
            .set_component(entity, component)
            .expect("Could not set component.");
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) {
        self.component_manager
            .remove_component::<C>(entity)
            .expect("Could not remove component from manager");
        let signature: Signature = C::get_kind().into();
        self.entity_manager
            .remove_signature(entity, signature)
            .expect("Could not remove signature.");
    }
}
