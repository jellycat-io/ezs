use eyre::Result;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::{Context, GameError, GameResult};

use config::Config;
use jecs::World;

use crate::components::color::Color;
use crate::components::human::Human;
use crate::components::visible_sight_range::VisibleSightRange;
use crate::components::{
    acceleration::Acceleration, location::Location, sight_range::SightRange, speed::Speed,
    velocity::Velocity,
};
use crate::data_structures::vector2::Vector2;
use crate::resources::{
    background_color::BackgroundColor, clicked_location::ClickedLocation, entity_mesh::EntityMesh,
    entity_size::EntitySize, fps::FPS,
};
use crate::systems::human_repulsion::HumanRepulsion;
use crate::systems::insert_zombie::InsertZombie;
use crate::systems::{
    contain_in_bounds::ContainInBounds, draw_entities::DrawEntities, randomly_walk::RandomlyWalk,
    reset_acceleration::ResetAcceleration, update_location::UpdateLocation,
    update_velocity::UpdateVelocity, visualize_sight_range::VisualizeSightRange,
};
use crate::utils::palette::Palette;

pub mod components;
pub mod config;
pub mod data_structures;
pub mod resources;
pub mod systems;
pub mod utils;

#[derive(Debug)]
pub struct MainState {
    world: World,
}

impl MainState {
    pub fn new(config: Config, ctx: &mut Context) -> Result<Self> {
        let mut world = World::new();
        let entity_size_resource = EntitySize::new(config.entity_size);
        world.add_resource(config.arena_size);
        world.add_resource(BackgroundColor(config.background_color));
        world.add_resource(ClickedLocation(None));
        world.add_resource(EntityMesh::new(config.entity_size, ctx)?);
        world.add_resource(entity_size_resource);
        world.add_resource(FPS(config.target_fps));

        world.register_component::<Location>();
        world.register_component::<Velocity>();
        world.register_component::<Acceleration>();
        world.register_component::<Speed>();
        world.register_component::<SightRange>();
        world.register_component::<Color>();
        world.register_component::<Human>();
        world.register_component::<VisibleSightRange>();

        for index in 0..config.humans_count {
            world
                .create_entity()
                .with_component(Location(Vector2::new_random_range(
                    entity_size_resource.half(),
                    config.arena_size.width - entity_size_resource.half(),
                )))?
                .with_component(Velocity::new())?
                .with_component(Acceleration::new())?
                .with_component(Speed(config.human_speed))?
                .with_component(SightRange(config.human_sight_range))?
                .with_component(Color(Palette::white()))?
                .with_component(Human)?;

            if index == 0 {
                world.add_component_by_entity_id(VisibleSightRange, index as usize)?;
            }
        }

        Ok(Self { world })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = **self.world.get_resource::<FPS>().unwrap();

        while ggez::timer::check_update_time(ctx, fps) {
            RandomlyWalk::run(&self.world).unwrap();
            UpdateVelocity::run(&self.world).unwrap();
            UpdateLocation::run(&self.world).unwrap();
            ResetAcceleration::run(&self.world).unwrap();
            ContainInBounds::run(&self.world).unwrap();
            HumanRepulsion::run(&self.world).unwrap();
            InsertZombie::run(&mut self.world).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bg_color = self
            .world
            .get_resource::<BackgroundColor>()
            .expect("Could not find background color");
        graphics::clear(ctx, **bg_color);
        DrawEntities::run(&self.world, ctx).unwrap();
        VisualizeSightRange::run(&self.world, ctx).unwrap();
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let clicked_location = self.world.get_resource_mut::<ClickedLocation>().unwrap();
        if let MouseButton::Left = button {
            clicked_location.set(x, y);
        }
    }
}
