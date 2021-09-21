use crate::components::location::Location;
use crate::components::velocity::Velocity;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct UpdateLocation;

impl UpdateLocation {
    pub fn run(world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Velocity>()?
            .run();

        let locations = query.result[0].clone();
        let velocities = query.result[1].clone();

        for (index, w_location) in locations.iter().enumerate() {
            let mut b_location = w_location.borrow_mut();
            let b_velocity = velocities[index].borrow();
            let location = b_location
                .downcast_mut::<Location>()
                .ok_or(JecsError::DowncastError)?;
            let velocity = b_velocity
                .downcast_ref::<Velocity>()
                .ok_or(JecsError::DowncastError)?;

            **location += **velocity;
        }

        Ok(())
    }
}
