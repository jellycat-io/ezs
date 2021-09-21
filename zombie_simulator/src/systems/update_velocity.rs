use crate::components::acceleration::Acceleration;
use crate::components::velocity::Velocity;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct UpdateVelocity;

impl UpdateVelocity {
    pub fn run(world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Velocity>()?
            .with_component::<Acceleration>()?
            .run();
        let velocities = query.result[0].clone();
        let accelerations = query.result[1].clone();

        for (index, w_acceleration) in accelerations.iter().enumerate() {
            let b_acceleration = w_acceleration.borrow();
            let mut b_velocity = velocities[index].borrow_mut();
            let acceleration = b_acceleration
                .downcast_ref::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;
            let velocity = b_velocity
                .downcast_mut::<Velocity>()
                .ok_or(JecsError::DowncastError)?;

            **velocity += **acceleration;
        }

        Ok(())
    }
}
