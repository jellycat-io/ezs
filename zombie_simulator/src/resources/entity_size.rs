#[derive(Debug)]
pub struct EntitySize(f32);

impl EntitySize {
	pub fn new(size: f32) -> Self {
		Self(size)
	}
}

