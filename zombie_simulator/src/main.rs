use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use ggez::{Context, ContextBuilder, GameError, GameResult};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("zombie_simulator", "Jellycat Studio")
        .build()
        .expect("Error creating GGEZ context");
    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}

struct Game {}

impl Game {
    pub fn new(__ctx: &mut Context) -> Self {
        Self {}
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [50.0, 50.0], 50.0, 0.1, Color::WHITE)?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        graphics::present(ctx)
    }
}
