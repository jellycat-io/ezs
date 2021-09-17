use crate::ezs_errors::EzsError;
use eyre::Result;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod query;

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    bit_masks: HashMap<TypeId, u32>,
    map: Vec<u32>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        let bit_mask = 2u32.pow(self.bit_masks.len() as u32);
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, bit_mask);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_, components)| components.push(None));
        self.map.push(0);

        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self> {
        let type_id = data.type_id();
        let map_index = self.map.len() - 1;
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or(EzsError::CreateEntityNeverCalled)?;
            *last_component = Some(Rc::new(RefCell::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map[map_index] |= *bit_mask;
        } else {
            return Err(EzsError::ComponentNotRegistered.into());
        }

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
            return Err(EzsError::ComponentNotRegistered.into());
        };

        self.map[index] ^= *mask;

        Ok(())
    }

    pub fn add_component_by_entity_id(&mut self, data: impl Any, index: usize) -> Result<()> {
        let mask = if let Some(mask) = self.bit_masks.get(&data.type_id()) {
            mask
        } else {
            return Err(EzsError::ComponentNotRegistered.into());
        };

        self.map[index] |= *mask;

        let components = self.components.get_mut(&data.type_id()).unwrap();
        components[index] = Some(Rc::new(RefCell::new(data)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn bitmask_updated_when_registering_entities() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let mask = entities.bit_masks.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(*mask, 1);

        entities.register_component::<Speed>();
        let mask = entities.bit_masks.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(*mask, 2);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
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
        let mut entities = Entities::default();
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
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(16.0))?;

        let entity_map = entities.map[0];
        assert_eq!(entity_map, 3);

        entities.create_entity().with_component(Speed(20.0))?;
        let entity_map = entities.map[1];
        assert_eq!(entity_map, 2);

        Ok(())
    }

    #[test]
    fn delete_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(16.0))?;

        entities.delete_component_by_entity_id::<Health>(0)?;

        assert_eq!(entities.map[0], 2);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn add_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity().with_component(Health(100))?;

        entities.add_component_by_entity_id(Speed(16.0), 0)?;

        assert_eq!(entities.map[0], 3);

        let speed_type_id = TypeId::of::<Speed>();
        let wrapped_speeds = entities.components.get(&speed_type_id).unwrap();
        let wrapped_speed = wrapped_speeds[0].as_ref().unwrap();
        let borrowed_speed = wrapped_speed.borrow();
        let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();

        assert_eq!(speed.0, 16.0);

        Ok(())
    }

    struct Health(pub u32);
    struct Speed(pub f32);
}
