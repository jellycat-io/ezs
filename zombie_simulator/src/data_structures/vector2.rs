use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl Add for Vector2 {
	type Output = Vector2;

	fn add(self, other: Self) -> Self::Output {
		Self { x: self.x + other.x, y: self.y + other.y }
	}
}

impl Sub for Vector2 {
	type Output = Vector2;

	fn sub(self, other: Self) -> Self::Output {
		Self { x: self.x - other.x, y: self.y - other.y }
	}
}