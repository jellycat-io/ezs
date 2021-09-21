use crate::components::acceleration::Acceleration;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct ResetAcceleration;

impl ResetAcceleration {
    pub fn run(world: &World) -> Result<()> {
        let query = world.query().with_component::<Acceleration>()?.run();

        let accelerations = query.result[0].clone();
        for w_acceleration in accelerations {
            let mut b_acceleration = w_acceleration.borrow_mut();
            let acceleration = b_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;

            acceleration.reset();
        }

        Ok(())
    }
}
