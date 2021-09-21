use zombie_simulator::MainState;
use ggez::graphics::Color;
use ggez::{ContextBuilder};
use ggez::event::{self};
use eyre::Result;
use ggez::conf::WindowMode;
use zombie_simulator::resources::arena_size::ArenaSize;

fn main() -> Result<()> {
    let window_mode = WindowMode::default().dimensions(1024.0, 1024.0);
    let (mut ctx, event_loop) = ContextBuilder::new("zombie_simulator", "jellycat-io")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context");

    let arena_size = ArenaSize::new(1024.0, 1024.0);
    let background_color = Color::from_rgb(29, 43, 83);
    let entity_size = 15.0;

    let main_state = MainState::new(arena_size, background_color, entity_size, 60, &mut ctx)?;

    event::run(ctx, event_loop, main_state);
}