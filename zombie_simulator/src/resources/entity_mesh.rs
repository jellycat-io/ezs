use ggez::graphics::{Mesh, MeshBuilder, DrawMode, Color};
use eyre::Result;
use ggez::Context;

#[derive(Debug)]
pub struct EntityMesh(Mesh);

impl EntityMesh {
	pub fn new(radius: f32, ctx: &mut Context) -> Result<Self> {
		let mesh = MeshBuilder::new()
			.circle(DrawMode::fill(), [0.0, 0.0], radius, 0.1, Color::WHITE)?
			.build(ctx)?;

		Ok(Self(mesh))
	}
}