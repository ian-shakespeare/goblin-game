use component_vec::ComponentVec;
use std::cell::{RefCell, RefMut};

mod component_vec;

const MAX_ENTITIES: usize = 5000;

pub type Entity = usize;

#[derive(Debug)]
pub enum EcsError {
    UnregisteredComponent,
    OutOfRange,
    TooManyEntities,
}

pub struct Ecs {
    entity_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn register_component<ComponentType: 'static>(&mut self) {
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entity_count);

        for _ in 0..self.entity_count {
            new_component_vec.push(None);
        }

        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn create_entity(&mut self) -> Result<Entity, EcsError> {
        if self.entity_count >= MAX_ENTITIES {
            return Err(EcsError::TooManyEntities);
        }
        let entity_id = self.entity_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entity_count += 1;

        Ok(entity_id)
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), EcsError> {
        if entity >= self.entity_count {
            return Err(EcsError::OutOfRange);
        }
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.remove(entity);
        }

        Ok(())
    }

    pub fn add_component<ComponentType: 'static>(
        &mut self,
        entity: Entity,
        component: ComponentType,
    ) -> Result<(), EcsError> {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);
                return Ok(());
            }
        }

        Err(EcsError::UnregisteredComponent)
    }

    pub fn get_component<ComponentType: 'static>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut Option<ComponentType>, EcsError> {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                if let Some(component) = component_vec.get_mut().get_mut(entity) {
                    return Ok(component);
                }
                return Err(EcsError::OutOfRange);
            }
        }

        Err(EcsError::UnregisteredComponent)
    }

    pub fn remove_component<ComponentType: 'static>(&mut self, entity: Entity) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = None;
            }
        }
    }

    pub fn get_component_vec<ComponentType: 'static>(
        &self,
    ) -> Result<RefMut<Vec<Option<ComponentType>>>, EcsError> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Ok(component_vec.borrow_mut());
            }
        }

        Err(EcsError::UnregisteredComponent)
    }
}
