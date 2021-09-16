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

    pub fn create_entity(&mut self) {
        self.components
            .iter_mut()
            .for_each(|(_, components)| components.push(None));
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

    struct Health(pub u32);
    struct Speed(pub f32);
}
