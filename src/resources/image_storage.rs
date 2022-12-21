use crate::prelude::*;

#[derive(Default, Resource)]
pub struct ImageStorageResource(pub Vec<Handle<Image>>);

impl ImageStorageResource {
    pub fn insert(mut self, value: Handle<Image>) -> Self {
        self.0.push(value);
        self
    }
}
