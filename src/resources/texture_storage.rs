use crate::prelude::*;

#[derive(Default, Resource)]
pub struct TextureStorageResource {
    pub textures: HashMap<String, Handle<Image>>,
}

impl TextureStorageResource {
    pub fn insert(mut self, key: String, value: Handle<Image>) -> Self {
        self.textures.insert(key, value);
        self
    }

    pub fn get(&self, key: &str) -> Option<Handle<Image>> { self.textures.get(key).cloned() }

    pub fn get_unchecked(&self, key: &str) -> Handle<Image> { self.textures.get(key).unwrap().clone() }
}
