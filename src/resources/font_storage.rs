use crate::prelude::*;

#[derive(Default, Resource)]
pub struct FontStorageResource(pub HashMap<String, (Handle<Font>, Vec<u8>)>);

impl FontStorageResource {
    pub fn load<P: AsRef<Path>>(mut self, asset_server: &AssetServer, key: &str, path: P) -> Self {
        let path = path.as_ref();

        let bytes = format!(
            "assets/{}",
            path.to_str().expect("Failed to convert path to string")
        )
        .load_raw()
        .expect("Failed to load font file");

        let font: Handle<Font> = asset_server.load(path);
        self.0.insert(key.to_owned(), (font, bytes));

        self
    }

    pub fn insert(mut self, key: &str, font: Handle<Font>, font_bytes: &[u8]) -> Self {
        self.0.insert(key.to_owned(), (font, font_bytes.to_vec()));
        self
    }

    pub fn get(&self, key: &str) -> Option<(Handle<Font>, Vec<u8>)> { self.0.get(key).cloned() }

    pub fn get_unchecked(&self, key: &str) -> (Handle<Font>, Vec<u8>) { self.get(key).unwrap() }

    pub fn get_font(&self, key: &str) -> Option<Handle<Font>> { self.get(key).map(|x| x.0) }

    pub fn get_font_unchecked(&self, key: &str) -> Handle<Font> { self.get_font(key).unwrap() }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> { self.get(key).map(|x| x.1) }

    pub fn get_bytes_unchecked(&self, key: &str) -> Vec<u8> { self.get_bytes(key).unwrap() }
}
