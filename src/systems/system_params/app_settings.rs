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

            pub fn get_window_resolution(&self) -> Vec2 { self.settings.window_resolution }

            pub fn get_window_mode(&self) -> WindowMode { self.settings.window_mode }

            pub fn set_grid_size(&mut self, value: UVec2) { self.settings.grid_size = value; }

            pub fn set_window_resolution(&mut self, value: Vec2) { self.settings.window_resolution = value; }

            pub fn set_window_mode(&mut self, value: WindowMode) { self.settings.window_mode = value; }

            pub fn save(&self) { self.settings.save(); }

            pub fn is_changed(&self) -> bool { self.settings.is_changed() }

            pub fn is_added(&self) -> bool { self.settings.is_added() }
        }
    };
}

impl_get_settings!(AppSettings);
