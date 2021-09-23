use eyre::Result;
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;

use jecs::errors::JecsError;
use jecs::World;

use crate::components::color::Color;
use crate::components::visible_sight_range::VisibleSightRange;
use crate::components::{location::Location, sight_range::SightRange};

pub struct VisualizeSightRange;

impl VisualizeSightRange {
    pub fn run(world: &World, ctx: &mut Context) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<SightRange>()?
            .with_component::<Color>()?
            .with_component::<VisibleSightRange>()?
            .run();

        let locations = query.result[0].clone();
        let sight_ranges = query.result[1].clone();
        let colors = query.result[2].clone();

        for (index, w_location) in locations.iter().enumerate() {
            let b_location = w_location.borrow();
            let b_sight_range = sight_ranges[index].borrow();
            let b_color = colors[index].borrow();
            let location = b_location
                .downcast_ref::<Location>()
                .ok_or(JecsError::DowncastError)?;
            let sight_range = b_sight_range
                .downcast_ref::<SightRange>()
                .ok_or(JecsError::DowncastError)?;
            let color = b_color
                .downcast_ref::<Color>()
                .ok_or(JecsError::DowncastError)?;

            let mesh = MeshBuilder::new()
                .circle(
                    DrawMode::stroke(1.0),
                    location.to_mint_vector2(),
                    **sight_range,
                    0.1,
                    **color,
                )?
                .build(ctx)?;

            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }

        Ok(())
    }
}
