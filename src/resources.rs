use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            data.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            data.downcast_mut()
        } else {
            None
        }
    }

    pub fn remove<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn add_resource() {
        let resources = initialize_resources();
        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();

        assert_eq!(extracted_world_width.0, 100.0);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resources();
        if let Some(extracted_world_width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(extracted_world_width.0, 100.0);
        }
    }

    #[test]
    fn get_resource_mutably() {
        let mut resources = initialize_resources();
        {
            let world_width: &mut WorldWidth = resources.get_mut::<WorldWidth>().unwrap();
            world_width.0 += 10.0;
        }
        let world_width = resources.get_ref::<WorldWidth>().unwrap();
        assert_eq!(world_width.0, 110.0);
    }

    #[test]
    fn remove_resource() {
        let mut resources = initialize_resources();
        resources.remove::<WorldWidth>();
        let world_width_type_id = TypeId::of::<WorldWidth>();
        assert!(!resources.data.contains_key(&world_width_type_id));
    }

    fn initialize_resources() -> Resources {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);
        resources
    }

    struct WorldWidth(pub f32);
}
