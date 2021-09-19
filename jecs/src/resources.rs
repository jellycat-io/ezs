use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, resource: impl Any) {
        self.data.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())?.downcast_mut::<T>()
    }

    pub fn remove<T: Any>(&mut self) {
        self.data.remove(&TypeId::of::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_resource() {
        let resources = initialize_resources();
        let health = resources.data
            .get(&TypeId::of::<Health>())
            .unwrap()
            .downcast_ref::<Health>()
            .unwrap();
        assert_eq!(health.0, 100);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resources();
        if let Some(health) = resources.get_ref::<Health>() {
            assert_eq!(health.0, 100);
        }
    }

    #[test]
    fn get_resource_mutably() {
        let mut resources = initialize_resources();
        {
            let health = resources.get_mut::<Health>().unwrap();
            health.0 += 10;
        }
        let health = resources.get_ref::<Health>().unwrap();
        assert_eq!(health.0, 110);
    }

    #[test]
    fn remove_resource() {
        let mut resources = initialize_resources();
        resources.remove::<Health>();
        assert!(!resources.data.contains_key(&TypeId::of::<Health>()));
    }

    fn initialize_resources() -> Resources {
        let mut resources = Resources::new();
        let health = Health::new(100);

        resources.add(health);
        resources
    }

    #[derive(Debug)]
    struct Health(pub u32);

    impl Health {
        pub fn new(health: u32) -> Self {
            Self(health)
        }
    }
}
