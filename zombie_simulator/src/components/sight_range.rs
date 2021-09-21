use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct SightRange(pub f32);

impl Deref for SightRange {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SightRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
