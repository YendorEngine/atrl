use crate::{prelude::*, types::resolution::*};

const APP_SETTINGS_PATH: &str = "./app_settings.toml";

const DEFAULT_GRID_SIZE: UVec2 = UVec2 { x: 80, y: 45 };

const DEFAULT_RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

const DEFAULT_WINDOW_MODE: WindowMode = WindowMode::Windowed;

#[derive(Resource, Clone)]
pub struct AppSettingsResource {
    pub grid_size: UVec2,
    pub window_mode: WindowMode,
    pub render_chunk_size: UVec2,
    pub window_resolution: Resolution,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppSettingsSerialized {
    grid_size: Option<UVec2>,
    window_mode: Option<WindowMode>,
    render_chunk_size: Option<UVec2>,
    window_resolution: Option<Resolution>,
}

// Save/Load
impl AppSettingsResource {
    pub fn load(default_fullscreen: bool) -> Self {
        if let Ok(s) = APP_SETTINGS_PATH.load_toml::<AppSettingsSerialized>() {
            s.into()
        } else {
            Self {
                window_mode: if default_fullscreen { WindowMode::Fullscreen } else { DEFAULT_WINDOW_MODE },
                ..Default::default()
            }
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
            window_mode: Some(value.window_mode),
            window_resolution: Some(value.window_resolution),
            render_chunk_size: Some(value.render_chunk_size),
        }
    }
}

impl From<AppSettingsSerialized> for AppSettingsResource {
    fn from(value: AppSettingsSerialized) -> Self {
        let grid_size = value.grid_size.unwrap_or(DEFAULT_GRID_SIZE);
        let window_mode = value.window_mode.unwrap_or(DEFAULT_WINDOW_MODE);
        let window_resolution = value.window_resolution.unwrap_or(Resolution::default());
        let render_chunk_size = value.render_chunk_size.unwrap_or(DEFAULT_RENDER_CHUNK_SIZE);

        Self {
            grid_size,
            window_mode,
            window_resolution,
            render_chunk_size,
        }
    }
}

impl Default for AppSettingsResource {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            window_mode: DEFAULT_WINDOW_MODE,
            window_resolution: Resolution::default(),
            render_chunk_size: DEFAULT_RENDER_CHUNK_SIZE,
        }
    }
}
