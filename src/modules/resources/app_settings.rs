use std::time::Duration;

use crate::prelude::{
    *,
    utilities::file::*,
};

const DEFAULT_UNSAFE_MS: u64 = 500;
const DEFAULT_REPEAT_MS: u64 = 500;
const DEFAULT_PRESSED_MS: u64 = 100;

const GAME_SETTINGS_FILE: &str = "app_settings.toml";

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct AppSettings {
    // Everything here needs to be Option so we can differentiate between Serialized or not. IF the settings
    // file does not have a setting defined, it will return back `None`.
    pressed_duration: Option<u64>,
    repeat_duration: Option<u64>,
    unsafe_duration: Option<u64>,
}

impl FromWorld for AppSettings {
    fn from_world(_world: &mut World) -> Self {
        // create with defaults
        let mut settings = Self::new();

        // load anything serialized
        let loaded_settings = Self::load_settings();

        // overwrite any default settings with the serialized settings
        if loaded_settings.pressed_duration.is_some() {
            settings.pressed_duration = loaded_settings.pressed_duration;
        }

        if loaded_settings.repeat_duration.is_some() {
            settings.repeat_duration = loaded_settings.repeat_duration;
        }

        if loaded_settings.unsafe_duration.is_some() {
            settings.unsafe_duration = loaded_settings.unsafe_duration;
        }

        // Return GameSettings
        info!("Game Settings: {settings:?}");
        settings
    }
}

impl AppSettings {
    #[inline(always)]
    const fn new() -> Self {
        // as these are our defaults, every one should be Some([default_value])
        Self {
            pressed_duration: Some(DEFAULT_PRESSED_MS),
            repeat_duration: Some(DEFAULT_REPEAT_MS),
            unsafe_duration: Some(DEFAULT_UNSAFE_MS),
        }
    }

    fn load_settings() -> Self {
        match read_toml::<Self>(GAME_SETTINGS_FILE) {
            Ok(settings) => settings,
            Err(err) => {
                error!("Unable to load settings: {err}");
                Self::new()
            },
        }
    }

    // Getters:
    // As we build from defaults in `Self::new()` every `self.Option` is guaranteed to be
    // `Some([value])` so unwrap the first `Option` in the getter
    pub fn pressed_duration(&self) -> Duration { Duration::from_millis(self.pressed_duration.unwrap()) }

    pub fn repeat_duration(&self) -> Duration { Duration::from_millis(self.repeat_duration.unwrap()) }

    pub fn unsafe_duration(&self) -> Duration { Duration::from_millis(self.unsafe_duration.unwrap()) }

    // Setters:
    // Set a value
    pub fn set_pressed_duration(&mut self, millis: u64) { self.pressed_duration = Some(millis); }

    pub fn set_repeat_duration(&mut self, millis: u64) { self.repeat_duration = Some(millis); }

    pub fn set_unsafe_duration(&mut self, millis: u64) { self.unsafe_duration = Some(millis); }
}
