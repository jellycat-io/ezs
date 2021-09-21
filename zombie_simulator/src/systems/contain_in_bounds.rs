use crate::components::location::Location;
use crate::components::velocity::Velocity;
use crate::resources::arena_size::ArenaSize;
use crate::resources::entity_size::EntitySize;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct ContainInBounds;

impl ContainInBounds {
    pub fn run(world: &World) -> Result<()> {
        let arena_size = world
            .get_resource::<ArenaSize>()
            .ok_or(JecsError::ResourceNotFound)?;
        let entity_size = world
            .get_resource::<EntitySize>()
            .ok_or(JecsError::ResourceNotFound)?;
        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Velocity>()?
            .run();

        let locations = query.result[0].clone();
        let velocities = query.result[1].clone();

        for (index, w_location) in locations.iter().enumerate() {
            let mut b_location = w_location.borrow_mut();
            let mut b_velocity = velocities[index].borrow_mut();
            let location = b_location
                .downcast_mut::<Location>()
                .ok_or(JecsError::DowncastError)?;
            let velocity = b_velocity
                .downcast_mut::<Velocity>()
                .ok_or(JecsError::DowncastError)?;

            if location.x - entity_size.half() < 0.0 {
                location.x = 0.0 + entity_size.half();
                velocity.reset_x();
            } else if location.x + entity_size.half() > arena_size.width {
                location.x = arena_size.width - entity_size.half();
                velocity.reset_x();
            }

            if location.y - entity_size.half() < 0.0 {
                location.y = 0.0 + entity_size.half();
                velocity.reset_y();
            } else if location.y + entity_size.half() > arena_size.width {
                location.y = arena_size.width - entity_size.half();
                velocity.reset_y();
            }
        }

        Ok(())
    }
}
