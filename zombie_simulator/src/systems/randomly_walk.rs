use crate::components::acceleration::Acceleration;
use crate::data_structures::vector2::Vector2;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct RandomlyWalk;

impl RandomlyWalk {
    pub fn run(world: &World) -> Result<()> {
        let query = world.query().with_component::<Acceleration>()?.run();

        let accelerations = &query.result[0];
        for w_acceleration in accelerations {
            let mut b_acceleration = w_acceleration.borrow_mut();
            let acceleration = b_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;

            let random_direction = Vector2::new_random_range(-0.1, 0.1);
            **acceleration += random_direction;
        }

        Ok(())
    }
}
