use eyre::Result;
use ggez::graphics::DrawParam;
use ggez::Context;

use jecs::errors::JecsError;
use jecs::World;

use crate::components::color::Color;
use crate::components::location::Location;
use crate::resources::entity_mesh::EntityMesh;

pub struct DrawEntities;

impl DrawEntities {
    pub fn run(world: &World, ctx: &mut Context) -> Result<()> {
        let mesh = world.get_resource::<EntityMesh>().unwrap();

        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Color>()?
            .run();

        let locations = query.result[0].clone();
        let colors = query.result[1].clone();

        for (index, w_location) in locations.iter().enumerate() {
            let b_location = w_location.borrow();
            let b_color = colors[index].borrow();
            let location = b_location
                .downcast_ref::<Location>()
                .ok_or(JecsError::DowncastError)?;
            let color = b_color
                .downcast_ref::<Color>()
                .ok_or(JecsError::DowncastError)?;

            ggez::graphics::draw(
                ctx,
                &mesh.0,
                DrawParam::new()
                    .dest(location.to_mint_vector2())
                    .color(**color),
            )?;
        }

        Ok(())
    }
}
