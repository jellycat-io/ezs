use ggez::graphics::Color;

pub struct Palette;

impl Palette {
    pub fn white() -> Color {
        Color::WHITE
    }

    pub fn black() -> Color {
        Color::BLACK
    }

    pub fn green() -> Color {
        Color::from_rgb(0, 228, 54)
    }

    pub fn light_grey() -> Color {
        Color::from_rgb(194, 195, 199)
    }

    pub fn dark_grey() -> Color {
        Color::from_rgb(95, 87, 79)
    }

    pub fn dark_blue() -> Color {
        Color::from_rgb(29, 43, 83)
    }
}
