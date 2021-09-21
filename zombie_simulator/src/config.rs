use ggez::graphics::Color;

use crate::resources::arena_size::ArenaSize;

#[derive(Debug)]
pub struct Config {
    pub arena_size: ArenaSize,
    pub background_color: Color,
    pub entity_size: f32,
    pub humans_count: u32,
    pub human_speed: f32,
    pub human_sight_range: f32,
    pub target_fps: u32,
}
