use crate::resources::Resources;
use std::any::Any;

mod resources;

#[derive(Default, Debug)]
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
}

#[cfg(test)]
mod tests {}
