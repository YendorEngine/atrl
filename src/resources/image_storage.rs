use crate::prelude::*;

#[derive(Default, Resource)]
pub struct ImageStorageResource(pub Vec<Handle<Image>>);

impl ImageStorageResource {
    pub fn insert(mut self, value: Handle<Image>) -> Self {
        self.0.push(value);
        self
    }

    pub fn get(&self, key: &str) -> Option<Handle<Image>> {
        self.0.iter().find(|x| x.id() == key.into()).cloned()
    }

    pub fn get_unchecked(&self, key: &str) -> Handle<Image> { self.get(key).unwrap() }
}
