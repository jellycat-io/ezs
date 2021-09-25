use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Speed(pub f32);

impl Speed {
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Deref for Speed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Speed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
