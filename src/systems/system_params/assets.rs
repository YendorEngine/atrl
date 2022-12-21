use crate::{
    prelude::*,
    resources::{font_storage::FontStorageResource, image_storage::ImageStorageResource},
};

#[derive(SystemParam)]
pub struct AppAssets<'w, 's> {
    fonts: ResMut<'w, FontStorageResource>,
    images: ResMut<'w, ImageStorageResource>,

    _phantom: Query<'w, 's, ()>,
}

impl<'w, 's> AppAssets<'w, 's> {
    pub fn get_font(&self, key: &str) -> Option<(Handle<Font>, Vec<u8>)> { self.fonts.get(key) }

    pub fn get_font_unchecked(&self, key: &str) -> (Handle<Font>, Vec<u8>) { self.get_font(key).unwrap() }

    pub fn is_fonts_changed(&self) -> bool { self.fonts.is_changed() }

    pub fn is_fonts_added(&self) -> bool { self.fonts.is_changed() }

    pub fn is_images_changed(&self) -> bool { self.images.is_added() }

    pub fn is_images_added(&self) -> bool { self.images.is_added() }
}
