use std::ops::{Deref, DerefMut};

use crate::data_structures::vector2::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Acceleration(pub Vector2);

impl Acceleration {
    pub fn new() -> Self {
        Self(Vector2::zero())
    }
}

impl Deref for Acceleration {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Acceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
