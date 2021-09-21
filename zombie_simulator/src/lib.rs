use jecs::World;
use eyre::Result;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult, GameError};
use ggez::event::EventHandler;

use crate::resources::{
	arena_size::ArenaSize,
	background_color::BackgroundColor,
	clicked_location::ClickedLocation,
	entity_size::EntitySize,
};
use crate::components::location::Location;
use crate::resources::entity_mesh::EntityMesh;
use crate::data_structures::vector2::Vector2;
use crate::systems::draw_entities::DrawEntities;

pub mod data_structures;
pub mod resources;
pub mod components;
pub mod systems;

#[derive(Debug)]
pub struct MainState {
	world: World
}

impl MainState {
	pub fn new(arena_size: ArenaSize, background_color: Color, entity_size: f32, ctx: &mut Context) -> Result<Self> {
		let mut world = World::new();
		world.add_resource(arena_size);
		world.add_resource(BackgroundColor(background_color));
		world.add_resource(ClickedLocation::new());
		world.add_resource(EntitySize::new(entity_size));
		world.add_resource(EntityMesh::new(entity_size, ctx)?);

		world.register_component::<Location>();
		world.create_entity()
			.with_component(Location(Vector2::new(30.0, 30.0)))?;

		Ok(Self { world })
	}
}

impl EventHandler<GameError> for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let bg_color = self.world.get_resource::<BackgroundColor>().expect("Could not find background color");
		graphics::clear(ctx, **bg_color);
		let draw_entities = DrawEntities;
		draw_entities.run(&self.world, ctx).unwrap();
		graphics::present(ctx)
	}
}