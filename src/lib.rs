use crate::resources::Resources;
use std::any::Any;

mod resources;

#[derive(Default, Debug)]
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /**
    Query for a resource and get a mutable reference to it.
    ```
    use ezs::World;

    let mut world = World::new();
    world.add_resource(10_u32);

    {
        let resource = world.get_resource_mut::<u32>().unwrap();
        *resource += 10;
    }

    let resource = world.get_resource::<u32>().unwrap();
    assert_eq!(*resource, 20);
    ```
    */
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
}

#[cfg(test)]
mod tests {}
