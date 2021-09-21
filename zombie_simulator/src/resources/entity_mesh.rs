use ggez::graphics::{Mesh, MeshBuilder, DrawMode, Color};
use eyre::Result;
use ggez::Context;
use std::ops::Deref;

#[derive(Debug)]
pub struct EntityMesh(pub Mesh);

impl EntityMesh {
	pub fn new(radius: f32, ctx: &mut Context) -> Result<Self> {
		let mesh = MeshBuilder::new()
			.circle(DrawMode::fill(), [0.0, 0.0], radius, 0.1, Color::WHITE)?
			.build(ctx)?;

		Ok(Self(mesh))
	}
}

impl Deref for EntityMesh {
	type Target = Mesh;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}