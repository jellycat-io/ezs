use eyre::Result;

use jecs::errors::JecsError;
use jecs::World;

use crate::components::acceleration::Acceleration;
use crate::components::human::Human;
use crate::components::location::Location;
use crate::components::sight_range::SightRange;
use crate::components::speed::Speed;
use crate::components::zombie::Zombie;

pub struct ZombieAttraction;

impl ZombieAttraction {
    pub fn run(world: &World) -> Result<()> {
        let z_query = world
            .query()
            .with_component::<Location>()?
            .with_component::<SightRange>()?
            .with_component::<Acceleration>()?
            .with_component::<Speed>()?
            .with_component::<Zombie>()?
            .run();

        let h_query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Human>()?
            .run();

        if z_query.result[0].is_empty() {
            return Ok(());
        }

        let z_locations = z_query.result[0].clone();
        let z_sight_ranges = z_query.result[1].clone();
        let z_speeds = z_query.result[3].clone();
        let z_accelerations = z_query.result[2].clone();
        let h_locations = h_query.result[0].clone();

        for (index, w_location) in z_locations.iter().enumerate() {
            let b_location = w_location.borrow();
            let z_location = b_location
                .downcast_ref::<Location>()
                .ok_or(JecsError::DowncastError)?;

            let b_sight_range = z_sight_ranges[index].borrow();
            let z_sight_range = b_sight_range
                .downcast_ref::<SightRange>()
                .ok_or(JecsError::DowncastError)?;

            let b_speed = z_speeds[index].borrow();
            let z_speed = b_speed
                .downcast_ref::<Speed>()
                .ok_or(JecsError::DowncastError)?;

            let mut b_acceleration = z_accelerations[index].borrow_mut();
            let z_acceleration = b_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or(JecsError::DowncastError)?;

            for wh_location in h_locations.iter() {
                let bh_location = wh_location.borrow();
                let h_location = bh_location
                    .downcast_ref::<Location>()
                    .ok_or(JecsError::DowncastError)?;

                if z_location.distance_to(h_location) < z_sight_range.get() {
                    let mut force = **h_location - z_location.get();
                    force.normalize();
                    force *= z_speed.get();
                    **z_acceleration += force;
                }
            }
        }

        Ok(())
    }
}
