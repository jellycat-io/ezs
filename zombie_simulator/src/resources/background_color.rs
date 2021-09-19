use ggez::graphics::Color;
use std::ops::Deref;

#[derive(Debug)]
pub struct BackgroundColor(pub Color);

impl Deref for BackgroundColor {
	type Target = Color;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}