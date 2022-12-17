use crate::{prelude::*, resources::AppSettingsResource};

#[derive(SystemParam)]
pub struct AppSettings<'w, 's> {
    _phantom: Query<'w, 's, ()>,
    settings: Res<'w, AppSettingsResource>,
}

#[derive(SystemParam)]
pub struct AppSettingsMut<'w, 's> {
    _phantom: Query<'w, 's, ()>,
    settings: ResMut<'w, AppSettingsResource>,
}

macro_rules! impl_get_settings {
    ($id:ident) => {
        impl<'w, 's> $id<'w, 's> {
            pub fn get_grid_size(&self) -> UVec2 { self.settings.get_grid_size() }

            pub fn get_cell_size(&self) -> UVec2 { self.settings.get_cell_size() }

            pub fn get_window_size(&self) -> UVec2 { self.settings.get_window_size() }

            pub fn get_fullscreen(&self) -> bool { self.settings.get_fullscreen() }
        }
    };
}

macro_rules! impl_set_settings {
    ($id:ident) => {
        impl<'w, 's> $id<'w, 's> {
            pub fn set_grid_size(&mut self, value: UVec2) { self.settings.set_grid_size(value); }

            pub fn set_cell_size(&mut self, value: UVec2) { self.settings.set_cell_size(value); }

            pub fn set_window_size(&mut self, value: UVec2) { self.settings.set_window_size(value); }

            pub fn set_fullscreen(&mut self, value: bool) { self.settings.set_fullscreen(value); }
        }
    };
}

impl_get_settings!(AppSettings);
impl_get_settings!(AppSettingsMut);
impl_set_settings!(AppSettingsMut);
