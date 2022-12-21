use crate::prelude::*;

#[derive(Default, Resource)]
pub struct FontStorageResource {
    pub fonts: HashMap<String, Handle<Font>>,
}

impl FontStorageResource {
    pub fn insert(mut self, key: String, value: Handle<Font>) -> Self {
        self.fonts.insert(key, value);
        self
    }

    pub fn get(&self, key: &str) -> Option<Handle<Font>> { self.fonts.get(key).cloned() }

    pub fn get_unchecked(&self, key: &str) -> Handle<Font> { self.fonts.get(key).unwrap().clone() }
}
