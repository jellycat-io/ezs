use jecs::World;
use eyre::Result;
use crate::resources::{
	arena_size::ArenaSize,
	background_color::BackgroundColor,
	clicked_location::ClickedLocation,
	entity_size::EntitySize,
};
use ggez::graphics::{self, Color};
use crate::resources::entity_mesh::EntityMesh;
use ggez::{Context, GameResult, GameError};
use ggez::event::EventHandler;

pub mod data_structures;
pub mod resources;

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
		graphics::present(ctx)
	}
}