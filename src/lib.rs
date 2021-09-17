use crate::entities::query::Query;
use crate::entities::Entities;
use crate::resources::Resources;
use eyre::Result;
use std::any::Any;

mod entities;
pub mod ezs_errors;
mod resources;

#[derive(Default, Debug)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Adds a resource to the world.
    The only restriction is that the resource type implements the Any trait.

    # Examples

    ```
    use ezs::World;
    let mut world = World::new();
    world.add_resource(10_u32);
    # assert_eq!(*world.get_resource::<u32>().unwrap(), 10);
    ```
     */
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    /**
    Queries for a resource and gets a reference to it.

    # Examples

    ```
    use ezs::World;

    let mut world = World::new();
    world.add_resource(10_u32);

    let resource = world.get_resource::<u32>().unwrap();
    # assert_eq!(*resource, 10);
    ```
     */
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /**
    Queries for a resource and gets a mutable reference to it.

    # Examples

    ```
    use ezs::World;

    let mut world = World::new();
    world.add_resource(10_u32);

    {
        let resource = world.get_resource_mut::<u32>().unwrap();
        *resource += 10;
    }

    let resource = world.get_resource::<u32>().unwrap();
    # assert_eq!(*resource, 20);
    ```
    */
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    ///Removes a resource from the world.
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    /// Registers a new empty component
    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>();
    }

    /**
    Creates an entity.\
    To pass components during the creation, use the `with_component` builder.\
    To add components to an entity, they have to be registered first using `register_component`.

    # Examples

    ```
    # use eyre::Result;
    use ezs::World;
    # fn try_main() -> Result<()> {
    let mut world = World::new();
    world.register_component::<u32>();
    world.register_component::<f32>();
    world
        .create_entity()
        .with_component(100_u32)?
        .with_component(16.0_f32)?;
    #    Ok(())
    # }
    # fn main() {
    #     try_main().unwrap();
    # }
    ```
    */

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }

    pub fn delete_component_by_entity_id<T: Any>(&mut self, index: usize) -> Result<()> {
        self.entities.delete_component_by_entity_id::<T>(index)
    }

    pub fn add_component_by_entity_id(&mut self, data: impl Any, index: usize) -> Result<()> {
        self.entities.add_component_by_entity_id(data, index)
    }

    pub fn delete_entity_by_id(&mut self, index: usize) -> Result<()> {
        self.entities.delete_entity_by_id(index)
    }
}

#[cfg(test)]
mod tests {}
