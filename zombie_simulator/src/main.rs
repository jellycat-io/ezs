use eyre::Result;
use ggez::conf::WindowMode;
use ggez::event::{self};
use ggez::ContextBuilder;

use zombie_simulator::config::Config;
use zombie_simulator::resources::arena_size::ArenaSize;
use zombie_simulator::utils::palette::Palette;
use zombie_simulator::MainState;

fn main() -> Result<()> {
    let window_mode = WindowMode::default().dimensions(1024.0, 1024.0);
    let (mut ctx, event_loop) = ContextBuilder::new("zombie_simulator", "jellycat-io")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context");

    let entity_size = 5.0;

    let config = Config {
        arena_size: ArenaSize::new(1024.0, 1024.0),
        background_color: Palette::black(),
        entity_size,
        humans_count: 100,
        human_speed: 0.15,
        human_sight_range: entity_size * 4.0,
        target_fps: 60,
    };

    let main_state = MainState::new(config, &mut ctx)?;

    event::run(ctx, event_loop, main_state);
}
