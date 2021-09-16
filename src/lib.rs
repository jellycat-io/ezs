use crate::resources::Resources;
use std::any::Any;

mod entities;
mod resources;

#[derive(Default, Debug)]
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Add a resource to the world.
    The only restriction is that the resource type implements the Any trait.
    ```
    use ezs::World;
    let mut world = World::new();
    world.add_resource(10_u32);
    assert_eq!(*world.get_resource::<u32>().unwrap(), 10);
    ```
     */
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    /**
    Query for a resource and get a reference to it.
    ```
    use ezs::World;

    let mut world = World::new();
    world.add_resource(10_u32);

    let resource = world.get_resource::<u32>().unwrap();
    assert_eq!(*resource, 10);
    ```
     */
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

    ///Remove a resource from the world.
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }
}

#[cfg(test)]
mod tests {}
