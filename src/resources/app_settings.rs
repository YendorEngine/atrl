use crate::prelude::*;

const APP_SETTINGS_PATH: &str = "./";

const DEFAULT_GRID_SIZE: UVec2 = UVec2 { x: 80, y: 45 };
const DEFAULT_CELL_SIZE: UVec2 = UVec2 { x: 24, y: 24 };
const DEFAULT_WINDOW_SIZE: UVec2 = UVec2 {
    x: DEFAULT_GRID_SIZE.x * DEFAULT_CELL_SIZE.x,
    y: DEFAULT_GRID_SIZE.y * DEFAULT_CELL_SIZE.y,
};
const DEFAULT_FULLSCREEN: bool = false;

#[derive(Resource, Clone)]
pub struct AppSettingsResource {
    grid_size: UVec2,
    cell_size: UVec2,
    window_size: UVec2,
    fullscreen: bool,
}

#[derive(Serialize, Deserialize)]
struct AppSettingsSerialized {
    grid_size: Option<UVec2>,
    cell_size: Option<UVec2>,
    window_size: Option<UVec2>,
    fullscreen: Option<bool>,
}

impl AppSettingsResource {
    pub fn get_grid_size(&self) -> UVec2 { self.grid_size }

    pub fn get_cell_size(&self) -> UVec2 { self.cell_size }

    pub fn get_window_size(&self) -> UVec2 { self.window_size }

    pub fn get_fullscreen(&self) -> bool { self.fullscreen }
}

impl AppSettingsResource {
    pub fn set_grid_size(&mut self, value: UVec2) { self.grid_size = value }

    pub fn set_cell_size(&mut self, value: UVec2) { self.cell_size = value }

    pub fn set_window_size(&mut self, value: UVec2) { self.window_size = value }

    pub fn set_fullscreen(&mut self, value: bool) { self.fullscreen = value }
}

impl From<AppSettingsResource> for AppSettingsSerialized {
    fn from(value: AppSettingsResource) -> Self {
        Self {
            grid_size: Some(value.grid_size),
            cell_size: Some(value.cell_size),
            window_size: Some(value.window_size),
            fullscreen: Some(value.fullscreen),
        }
    }
}

impl From<AppSettingsSerialized> for AppSettingsResource {
    fn from(value: AppSettingsSerialized) -> Self {
        let grid_size = value.grid_size.unwrap_or(DEFAULT_GRID_SIZE);
        let cell_size = value.cell_size.unwrap_or(DEFAULT_CELL_SIZE);
        let window_size = value.window_size.unwrap_or(DEFAULT_WINDOW_SIZE);
        let fullscreen = value.fullscreen.unwrap_or(DEFAULT_FULLSCREEN);

        Self {
            grid_size,
            cell_size,
            window_size,
            fullscreen,
        }
    }
}

impl Default for AppSettingsResource {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            cell_size: DEFAULT_CELL_SIZE,
            window_size: DEFAULT_WINDOW_SIZE,
            fullscreen: DEFAULT_FULLSCREEN,
        }
    }
}

impl AppSettingsResource {
    pub fn load() -> Self {
        if let Ok(s) = APP_SETTINGS_PATH.from_toml::<AppSettingsSerialized>() {
            s.into()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let serialized = AppSettingsSerialized::from(self.clone());
        if let Err(e) = APP_SETTINGS_PATH.to_toml(&serialized) {
            error!("{}", e);
        }
    }
}
