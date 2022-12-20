use crate::{prelude::*, resources::app_settings::AppSettingsResource};

#[derive(SystemParam)]
pub struct AppSettings<'w, 's> {
    settings: ResMut<'w, AppSettingsResource>,

    _phantom: Query<'w, 's, ()>,
}

macro_rules! impl_get_settings {
    ($id:ident) => {
        impl<'w, 's> $id<'w, 's> {
            pub fn get_grid_size(&self) -> UVec2 { self.settings.grid_size }

            pub fn get_window_size(&self) -> Vec2 { self.settings.window_size }

            pub fn get_fullscreen(&self) -> bool { self.settings.fullscreen }

            pub fn set_grid_size(&mut self, value: UVec2) { self.settings.grid_size = value; }

            pub fn set_window_size(&mut self, value: Vec2) { self.settings.window_size = value; }

            pub fn set_fullscreen(&mut self, value: bool) { self.settings.fullscreen = value; }

            pub fn save(&self) { self.settings.save(); }

            pub fn is_changed(&self) -> bool { self.settings.is_changed() }

            pub fn is_added(&self) -> bool { self.settings.is_added() }
        }
    };
}

impl_get_settings!(AppSettings);
