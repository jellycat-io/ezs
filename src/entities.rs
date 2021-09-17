use eyre::{bail, Result};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
    pub fn register_component<T: Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_, components)| components.push(None));
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self> {
        let type_id = data.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or("Could not find last component")
                .unwrap();
            *last_component = Some(Rc::new(RefCell::new(data)));
        } else {
            bail!("Attempted to insert data for a not registered component");
        }

        Ok(self)
    }
}

#[cfg(test)]
mod test {
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

    struct Health(pub u32);
    struct Speed(pub f32);
}
