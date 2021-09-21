use std::ops::Deref;

#[derive(Debug)]
pub struct FPS(pub u32);

impl Deref for FPS {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
