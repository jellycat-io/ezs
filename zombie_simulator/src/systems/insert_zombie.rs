use jecs::World;

use crate::components::color::Color;
use crate::components::location::Location;
use crate::resources::clicked_location::ClickedLocation;
use crate::utils::palette::Palette;

pub struct InsertZombie;

impl InsertZombie {
    pub fn run(world: &mut World) {
        let clicked_location = world.get_resource_mut::<ClickedLocation>().unwrap();
        let location = if let Some(location) = **clicked_location {
            location
        } else {
            return;
        };

        clicked_location.clear();
        world
            .create_entity()
            .with_component(Location(location))
            .unwrap()
            .with_component(Color(Palette::green()))
            .unwrap();
    }
}
