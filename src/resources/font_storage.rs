use crate::prelude::*;

#[derive(Default, Resource)]
pub struct FontStorageResource(pub Vec<Handle<Font>>);

impl FontStorageResource {
    pub fn insert(mut self, value: Handle<Font>) -> Self {
        self.0.push(value);
        self
    }

    pub fn get(&self, key: &str) -> Option<Handle<Font>> {
        self.0.iter().find(|x| x.id() == key.into()).cloned()
    }

    pub fn get_unchecked(&self, key: &str) -> Handle<Font> { self.get(key).unwrap() }
}
