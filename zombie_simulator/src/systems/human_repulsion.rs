use eyre::Result;

use jecs::errors::JecsError;
use jecs::World;

use crate::components::acceleration::Acceleration;
use crate::components::human::Human;
use crate::components::location::Location;
use crate::components::sight_range::SightRange;
use crate::components::speed::Speed;

pub struct HumanRepulsion;

impl HumanRepulsion {
    pub fn run(world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Speed>()?
            .with_component::<Acceleration>()?
            .with_component::<SightRange>()?
            .with_component::<Human>()?
            .run();

        let locations = query.result[0].clone();
        let speeds = query.result[1].clone();
        let accelerations = query.result[2].clone();
        let sight_ranges = query.result[3].clone();

        for (entity_index, w_entity_location) in locations.iter().enumerate() {
            let b_entity_location = w_entity_location.borrow();
            let b_entity_sight_range = sight_ranges[entity_index].borrow();
            let b_entity_speed = speeds[entity_index].borrow();
            let mut b_entity_acceleration = accelerations[entity_index].borrow_mut();
            let entity_speed = b_entity_speed
                .downcast_ref::<Speed>()
                .ok_or(JecsError::DowncastError)?;
            let entity_location = b_entity_location
                .downcast_ref::<Location>()
                .ok_or(JecsError::DowncastError)?;
            let entity_sight_range = b_entity_sight_range
                .downcast_ref::<SightRange>()
                .ok_or(JecsError::DowncastError)?;
            let entity_acceleration = b_entity_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;

            for (other_index, w_other_location) in locations.iter().enumerate() {
                if entity_index == other_index {
                    continue;
                }

                let b_other_location = w_other_location.borrow();
                let other_location = b_other_location
                    .downcast_ref::<Location>()
                    .ok_or(JecsError::DowncastError)?;

                if entity_location.distance_to(&**other_location) < **entity_sight_range {
                    let mut force = **entity_location - &**other_location;
                    force.normalize();
                    force *= **entity_speed;
                    **entity_acceleration += force;
                }
            }
        }

        Ok(())
    }
}
