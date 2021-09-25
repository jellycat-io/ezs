use eyre::Result;

use jecs::World;

use crate::components::acceleration::Acceleration;
use crate::components::color::Color;
use crate::components::location::Location;
use crate::components::sight_range::SightRange;
use crate::components::speed::Speed;
use crate::components::velocity::Velocity;
use crate::components::visible_sight_range::VisibleSightRange;
use crate::components::zombie::Zombie;
use crate::resources::clicked_location::ClickedLocation;
use crate::utils::palette::Palette;

const ZOMBIE_SPEED: f32 = 0.05;
const ZOMBIE_SIGHT_RANGE: f32 = 50.0;

pub struct InsertZombie;

impl InsertZombie {
    pub fn run(world: &mut World) -> Result<()> {
        let clicked_location = world.get_resource_mut::<ClickedLocation>().unwrap();
        let location = if let Some(location) = **clicked_location {
            location
        } else {
            return Ok(());
        };

        clicked_location.clear();
        world
            .create_entity()
            .with_component(Location(location))?
            .with_component(Color(Palette::green()))?
            .with_component(Speed(ZOMBIE_SPEED))?
            .with_component(Acceleration::new())?
            .with_component(Velocity::new())?
            .with_component(SightRange(ZOMBIE_SIGHT_RANGE))?
            .with_component(VisibleSightRange)?
            .with_component(Zombie)?;

        Ok(())
    }
}
