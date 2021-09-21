use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct EntitySize(f32);

impl EntitySize {
    pub fn new(size: f32) -> Self {
        Self(size)
    }

    pub fn half(&self) -> f32 {
        self.0 / 2.0
    }
}

impl Deref for EntitySize {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
