use crate::data_structures::vector2::Vector2;

#[derive(Debug, Default)]
pub struct ClickedLocation {
	pub location: Option<Vector2>
}

impl ClickedLocation {
	pub fn new() -> Self {
		Self::default()
	}
}