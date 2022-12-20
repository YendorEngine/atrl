use crate::prelude::*;

const APP_SETTINGS_PATH: &str = "./app_settings.toml";

const DEFAULT_GRID_SIZE: UVec2 = UVec2 { x: 80, y: 45 };
const DEFAULT_WINDOW_SIZE: Vec2 = Vec2 {
    x: 1280.0,
    y: 720.0,
};

const DEFAULT_RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

const DEFAULT_FULLSCREEN: bool = false;

#[derive(Resource, Clone)]
pub struct AppSettingsResource {
    pub grid_size: UVec2,
    pub window_size: Vec2,

    pub render_chunk_size: UVec2,

    pub fullscreen: bool,
}

#[derive(Serialize, Deserialize)]
struct AppSettingsSerialized {
    grid_size: Option<UVec2>,
    window_size: Option<Vec2>,

    render_chunk_size: Option<UVec2>,

    fullscreen: Option<bool>,
}

// Save/Load
impl AppSettingsResource {
    pub fn load() -> Self {
        if let Ok(s) = APP_SETTINGS_PATH.load_toml::<AppSettingsSerialized>() {
            s.into()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let serialized = AppSettingsSerialized::from(self.clone());
        if let Err(e) = APP_SETTINGS_PATH.to_toml(&serialized) {
            error!(
                "Error creating {:?}:\n{}",
                APP_SETTINGS_PATH.path_string(),
                e
            );
        }
    }
}

impl From<AppSettingsResource> for AppSettingsSerialized {
    fn from(value: AppSettingsResource) -> Self {
        Self {
            grid_size: Some(value.grid_size),
            window_size: Some(value.window_size),
            render_chunk_size: Some(value.render_chunk_size),
            fullscreen: Some(value.fullscreen),
        }
    }
}

impl From<AppSettingsSerialized> for AppSettingsResource {
    fn from(value: AppSettingsSerialized) -> Self {
        let grid_size = value.grid_size.unwrap_or(DEFAULT_GRID_SIZE);
        let window_size = value.window_size.unwrap_or(DEFAULT_WINDOW_SIZE);
        let render_chunk_size = value.render_chunk_size.unwrap_or(DEFAULT_RENDER_CHUNK_SIZE);
        let fullscreen = value.fullscreen.unwrap_or(DEFAULT_FULLSCREEN);

        Self {
            grid_size,
            window_size,
            render_chunk_size,
            fullscreen,
        }
    }
}

impl Default for AppSettingsResource {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            window_size: DEFAULT_WINDOW_SIZE,
            render_chunk_size: DEFAULT_RENDER_CHUNK_SIZE,
            fullscreen: DEFAULT_FULLSCREEN,
        }
    }
}
