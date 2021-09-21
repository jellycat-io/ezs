use eyre::Result;
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;

use jecs::errors::JecsError;
use jecs::World;

use crate::components::{location::Location, sight_range::SightRange};

pub struct VisualizeSightRange;

impl VisualizeSightRange {
    pub fn run(world: &World, ctx: &mut Context) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<SightRange>()?
            .run();

        if let Some(w_location) = query.result[0].first() {
            let b_location = w_location.borrow();
            let location = b_location
                .downcast_ref::<Location>()
                .ok_or(JecsError::DowncastError)?;

            if let Some(w_sight_range) = query.result[1].first() {
                let b_sight_range = w_sight_range.borrow();
                let sight_range = b_sight_range
                    .downcast_ref::<SightRange>()
                    .ok_or(JecsError::DowncastError)?;

                let mesh = MeshBuilder::new()
                    .circle(
                        DrawMode::stroke(1.0),
                        location.to_mint_vector2(),
                        **sight_range,
                        0.1,
                        Color::from_rgb(194, 195, 199),
                    )?
                    .build(ctx)?;

                graphics::draw(ctx, &mesh, DrawParam::new())?;
            }
        }

        Ok(())
    }
}
