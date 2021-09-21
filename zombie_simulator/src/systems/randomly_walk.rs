use crate::components::acceleration::Acceleration;
use crate::components::speed::Speed;
use crate::data_structures::vector2::Vector2;
use eyre::Result;
use jecs::errors::JecsError;
use jecs::World;

pub struct RandomlyWalk;

impl RandomlyWalk {
    pub fn run(world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Acceleration>()?
            .with_component::<Speed>()?
            .run();

        let accelerations = query.result[0].clone();
        let speeds = query.result[1].clone();
        for (index, w_acceleration) in accelerations.iter().enumerate() {
            let mut b_acceleration = w_acceleration.borrow_mut();
            let b_speed = speeds[index].borrow();
            let acceleration = b_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;
            let speed = b_speed
                .downcast_ref::<Speed>()
                .ok_or(JecsError::DowncastError)?;

            let random_direction = Vector2::new_random_range(-**speed, **speed);
            **acceleration += random_direction;
        }

        Ok(())
    }
}
