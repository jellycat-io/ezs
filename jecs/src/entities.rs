use crate::errors::JecsError;
use eyre::Result;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod query;

pub type Component = Rc<RefCell<dyn Any>>;
pub type Components = HashMap<TypeId, Vec<Option<Component>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: Components,
    bit_masks: HashMap<TypeId, u32>,
    bitmap: Vec<u32>,
    inserting_into_index: usize,
}

impl Entities {
    pub fn new() -> Self { Self::default() }

    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        let bit_mask = 2u32.pow(self.bit_masks.len() as u32);
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, bit_mask);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        if let Some((index, _)) = self
            .bitmap
            .iter()
            .enumerate()
            .find(|(_index, mask)| **mask == 0)
        {
            self.inserting_into_index = index;
        } else {
            self.components
                .iter_mut()
                .for_each(|(_, components)| components.push(None));
            self.bitmap.push(0);
            self.inserting_into_index = self.bitmap.len() - 1;
        }

        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self> {
        let type_id = data.type_id();
        let index = self.inserting_into_index;
        if let Some(components) = self.components.get_mut(&type_id) {
            let component = components
                .get_mut(index)
                .ok_or(JecsError::CreateEntityNeverCalled)?;
            *component = Some(Rc::new(RefCell::new(data)));
        } else {
            return Err(JecsError::ComponentNotRegistered.into());
        }

        let bit_mask = self.bit_masks.get(&type_id).unwrap();
        self.bitmap[index] |= *bit_mask;

        Ok(self)
    }

    pub fn get_bit_mask(&self, type_id: &TypeId) -> Option<u32> {
        self.bit_masks.get(type_id).copied()
    }

    pub fn delete_component_by_entity_id<T: Any>(&mut self, index: usize) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id) {
            mask
        } else {
            return Err(JecsError::ComponentNotRegistered.into());
        };

        self.bitmap[index] ^= *mask;

        Ok(())
    }

    pub fn add_component_by_entity_id(&mut self, data: impl Any, index: usize) -> Result<()> {
        let mask = if let Some(mask) = self.bit_masks.get(&data.type_id()) {
            mask
        } else {
            return Err(JecsError::ComponentNotRegistered.into());
        };

        self.bitmap[index] |= *mask;

        let components = self.components.get_mut(&data.type_id()).unwrap();
        components[index] = Some(Rc::new(RefCell::new(data)));

        Ok(())
    }

    pub fn delete_entity_by_id(&mut self, index: usize) -> Result<()> {
        if let Some(map) = self.bitmap.get_mut(index) {
            *map = 0;
        } else {
            return Err(JecsError::EntityDoesNotExist.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn register_component() {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn bitmask_updated_when_registering_entities() {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        let mask = entities.bit_masks.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(*mask, 1);

        entities.register_component::<Speed>();
        let mask = entities.bit_masks.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(*mask, 2);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();

        let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(health.len(), 1);
        assert!(health[0].is_none());
        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(speed.len(), 1);
        assert!(speed[0].is_none());
    }

    #[test]
    fn with_component() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(16.0))?;

        let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
        let wrapped_health = first_health.as_ref().unwrap();
        let borrowed_health = wrapped_health.borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();
        assert_eq!(health.0, 100);

        Ok(())
    }

    #[test]
    fn map_is_updated_when_creating_entities() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(16.0))?;

        let entity_map = entities.bitmap[0];
        assert_eq!(entity_map, 3);

        entities.create_entity().with_component(Speed(20.0))?;
        let entity_map = entities.bitmap[1];
        assert_eq!(entity_map, 2);

        Ok(())
    }

    #[test]
    fn delete_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(16.0))?;

        entities.delete_component_by_entity_id::<Health>(0)?;

        assert_eq!(entities.bitmap[0], 2);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn add_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity().with_component(Health(100))?;

        entities.add_component_by_entity_id(Speed(16.0), 0)?;

        assert_eq!(entities.bitmap[0], 3);

        let wrapped_speeds = entities.components.get(&TypeId::of::<Speed>()).unwrap();
        let wrapped_speed = wrapped_speeds[0].as_ref().unwrap();
        let borrowed_speed = wrapped_speed.borrow();
        let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();

        assert_eq!(speed.0, 16.0);

        Ok(())
    }

    #[test]
    fn delete_entity_by_id() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.create_entity().with_component(Health(100))?;
        entities.delete_entity_by_id(0)?;
        assert_eq!(entities.bitmap[0], 0);
        Ok(())
    }

    #[test]
    fn created_entities_are_inserted_into_deleted_entities_columns() -> Result<()> {
        let mut entities = Entities::new();
        entities.register_component::<Health>();
        entities.create_entity().with_component(Health(100))?;
        entities.create_entity().with_component(Health(200))?;
        entities.delete_entity_by_id(0)?;
        entities.create_entity().with_component(Health(300))?;

        assert_eq!(entities.bitmap[0], 1);

        let borrowed_health = entities.components.get(&TypeId::of::<Health>()).unwrap()[0]
            .as_ref()
            .unwrap()
            .borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();

        assert_eq!(health.0, 300);

        Ok(())
    }

    struct Health(pub u32);
    struct Speed(pub f32);
}
