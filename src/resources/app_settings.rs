use crate::{prelude::*, utilities::*};

// File
const APP_SETTINGS_FILE: &str = "app_settings.toml";

// Defaults
const DEFAULT_GRID_SIZE: UVec2 = UVec2 { x: 16, y: 9 };
const DEFAULT_SPRITE_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
const DEFAULT_WINDOW_SIZE: UVec2 = UVec2 {
    x: DEFAULT_GRID_SIZE.x * DEFAULT_SPRITE_SIZE.x,
    y: DEFAULT_GRID_SIZE.y * DEFAULT_SPRITE_SIZE.y,
};
const DEFAULT_FULLSCREEN: bool = false;

const DEFAULT_CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
const DEFAULT_TILE_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

#[derive(Resource)]
pub struct AppSettingsResource {
    grid_size: UVec2,
    sprite_size: UVec2,
    window_size: UVec2,
    fullscreen: bool,
    chunk_size: UVec2,
    tile_size: UVec2,
}

impl AppSettingsResource {
    /// loads the saved file, any missing sections are filled in with defaults.
    pub fn load() -> Self {
        file::read_toml::<AppSettingsSerialized>(APP_SETTINGS_FILE).map_or_else(
            |err| {
                error!("Error reading app_settings.toml: {err}");
                Self::default()
            },
            Self::from,
        )
    }

    /// Saves the current settings.
    pub fn save(self) {
        let serialized = AppSettingsSerialized::from(self);
        file::write_toml(APP_SETTINGS_FILE, &serialized).map_or_else(
            |err| error!("Error writing app_settings.toml: {err}"),
            |_| (),
        )
    }

    // Getters
    #[inline]
    pub const fn get_grid_size(&self) -> UVec2 { self.grid_size }

    #[inline]
    pub const fn get_sprite_size(&self) -> UVec2 { self.sprite_size }

    #[inline]
    pub const fn get_window_size(&self) -> UVec2 { self.window_size }

    #[inline]
    pub const fn get_fullscreen(&self) -> bool { self.fullscreen }

    #[inline]
    pub const fn get_chunk_size(&self) -> UVec2 { self.chunk_size }

    #[inline]
    pub const fn get_tile_size(&self) -> TilemapTileSize {
        TilemapTileSize {
            x: self.tile_size.x as f32,
            y: self.tile_size.y as f32,
        }
    }

    // Setters
    #[inline]
    pub fn set_grid_size(&mut self, grid_size: UVec2) { self.grid_size = grid_size; }

    #[inline]
    pub fn set_sprite_size(&mut self, sprite_size: UVec2) { self.sprite_size = sprite_size; }

    #[inline]
    pub fn set_window_size(&mut self, window_size: UVec2) { self.window_size = window_size; }

    #[inline]
    pub fn set_fullscreen(&mut self, fullscreen: bool) { self.fullscreen = fullscreen }

    #[inline]
    pub fn set_chunk_size(&mut self, chunk_size: UVec2) { self.chunk_size = chunk_size; }

    #[inline]
    pub fn set_tile_size(&mut self, tile_size: TilemapTileSize) {
        self.tile_size = UVec2::new(tile_size.x as u32, tile_size.y as u32);
    }
}

impl Default for AppSettingsResource {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            sprite_size: DEFAULT_SPRITE_SIZE,
            window_size: DEFAULT_WINDOW_SIZE,
            fullscreen: DEFAULT_FULLSCREEN,
            chunk_size: DEFAULT_CHUNK_SIZE,
            tile_size: DEFAULT_TILE_SIZE,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AppSettingsSerialized {
    grid_size: Option<UVec2>,
    sprite_size: Option<UVec2>,
    window_size: Option<UVec2>,
    fullscreen: Option<bool>,
    chunk_size: Option<UVec2>,
    tile_size: Option<UVec2>,
}

impl From<AppSettingsResource> for AppSettingsSerialized {
    fn from(value: AppSettingsResource) -> Self {
        Self {
            grid_size: Some(value.grid_size),
            sprite_size: Some(value.sprite_size),
            window_size: Some(value.window_size),
            fullscreen: Some(value.fullscreen),
            chunk_size: Some(value.chunk_size),
            tile_size: Some(value.tile_size),
        }
    }
}

impl From<AppSettingsSerialized> for AppSettingsResource {
    fn from(value: AppSettingsSerialized) -> Self {
        let grid_size = value.grid_size.unwrap_or(DEFAULT_GRID_SIZE);
        let sprite_size = value.sprite_size.unwrap_or(DEFAULT_SPRITE_SIZE);
        let window_size = value.window_size.unwrap_or(DEFAULT_WINDOW_SIZE);
        let fullscreen = value.fullscreen.unwrap_or(DEFAULT_FULLSCREEN);
        let chunk_size = value.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        let tile_size = value.tile_size.unwrap_or(DEFAULT_TILE_SIZE);

        Self {
            grid_size,
            sprite_size,
            window_size,
            fullscreen,
            chunk_size,
            tile_size,
        }
    }
}
