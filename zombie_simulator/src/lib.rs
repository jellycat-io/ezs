use crate::components::speed::Speed;
use crate::components::{acceleration::Acceleration, location::Location, velocity::Velocity};
use crate::data_structures::vector2::Vector2;
use crate::resources::fps::FPS;
use crate::resources::{
    arena_size::ArenaSize, background_color::BackgroundColor, clicked_location::ClickedLocation,
    entity_mesh::EntityMesh, entity_size::EntitySize,
};
use crate::systems::contain_in_bounds::ContainInBounds;
use crate::systems::randomly_walk::RandomlyWalk;
use crate::systems::reset_acceleration::ResetAcceleration;
use crate::systems::update_location::UpdateLocation;
use crate::systems::{draw_entities::DrawEntities, update_velocity::UpdateVelocity};
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{Context, GameError, GameResult};
use jecs::World;

pub mod components;
pub mod data_structures;
pub mod resources;
pub mod systems;

#[derive(Debug)]
pub struct MainState {
    world: World,
}

impl MainState {
    pub fn new(
        arena_size: ArenaSize,
        background_color: Color,
        entity_size: f32,
        humans_count: u32,
        target_fps: u32,
        ctx: &mut Context,
    ) -> Result<Self> {
        let mut world = World::new();
        let entity_size_resource = EntitySize::new(entity_size);
        world.add_resource(arena_size);
        world.add_resource(BackgroundColor(background_color));
        world.add_resource(ClickedLocation::new());
        world.add_resource(EntityMesh::new(entity_size, ctx)?);
        world.add_resource(entity_size_resource);
        world.add_resource(FPS(target_fps));

        world.register_component::<Location>();
        world.register_component::<Velocity>();
        world.register_component::<Acceleration>();
        world.register_component::<Speed>();

        for _ in 0..humans_count {
            world
                .create_entity()
                .with_component(Location(Vector2::new_random_range(
                    entity_size_resource.half(),
                    arena_size.width - entity_size_resource.half(),
                )))?
                .with_component(Velocity(Vector2::zero()))?
                .with_component(Acceleration(Vector2::zero()))?
                .with_component(Speed(0.3))?;
        }

        Ok(Self { world })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = self.world.get_resource::<FPS>().unwrap();

        while ggez::timer::check_update_time(ctx, **fps) {
            RandomlyWalk::run(&self.world).unwrap();
            UpdateVelocity::run(&self.world).unwrap();
            UpdateLocation::run(&self.world).unwrap();
            ContainInBounds::run(&self.world).unwrap();
            ResetAcceleration::run(&self.world).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bg_color = self
            .world
            .get_resource::<BackgroundColor>()
            .expect("Could not find background color");
        graphics::clear(ctx, **bg_color);
        let draw_entities = DrawEntities;
        draw_entities.run(&self.world, ctx).unwrap();
        graphics::present(ctx)
    }
}
