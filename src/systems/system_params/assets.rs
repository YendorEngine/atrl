use crate::{
    prelude::*,
    resources::{font_storage::FontStorageResource, texture_storage::TextureStorageResource},
};

#[derive(SystemParam)]
pub struct AppAssets<'w, 's> {
    fonts: ResMut<'w, FontStorageResource>,
    textures: ResMut<'w, TextureStorageResource>,

    _phantom: Query<'w, 's, ()>,
}

impl<'w, 's> AppAssets<'w, 's> {
    pub fn get_font(&self, key: &str) -> Option<Handle<Font>> { self.fonts.get(key) }

    pub fn get_texture(&self, key: &str) -> Option<Handle<Image>> { self.textures.get(key) }

    pub fn get_font_unchecked(&self, key: &str) -> Handle<Font> { self.fonts.get_unchecked(key) }

    pub fn get_texture_unchecked(&self, key: &str) -> Handle<Image> { self.textures.get_unchecked(key) }

    pub fn is_fonts_changed(&self) -> bool { self.fonts.is_changed() }

    pub fn is_fonts_added(&self) -> bool { self.fonts.is_changed() }

    pub fn is_textures_changed(&self) -> bool { self.textures.is_added() }

    pub fn is_textures_added(&self) -> bool { self.textures.is_added() }
}
