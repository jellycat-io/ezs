use crate::data_structures::vector2::Vector2;
use std::ops::Deref;

#[derive(Debug)]
pub struct Location(pub Vector2);

impl Deref for Location {
	type Target = Vector2;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}