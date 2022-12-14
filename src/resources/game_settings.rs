use std::time::Duration;

use crate::{modules::utilities::*, prelude::*};

const DEFAULT_UNSAFE_MS: u64 = 500;
const DEFAULT_PRESSED_MS: u64 = 150;
const GAME_SETTINGS_FILE: &str = "assets/raws/settings/game_settings.toml";

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct GameSettings {
    // Everything here needs to be Option so we can differentiate between Serialized or not. IF the settings
    // file does not have a setting defined, it will return back `None`.
    pressed_duration: Option<u64>,
    unsafe_duration: Option<u64>,
}

impl Default for GameSettings {
    fn default() -> Self { Self::extend_settings(Self::new(), Self::load_settings()) }
}

impl std::fmt::Display for GameSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameSettings {{ PressedDuration: {}, UnsafeDuration: {} }}",
            self.pressed_duration.unwrap(),
            self.unsafe_duration.unwrap()
        )
    }
}

impl GameSettings {
    #[inline(always)]
    const fn new() -> Self {
        // as these are our defaults, every one should be Some([default_value])
        Self {
            pressed_duration: Some(DEFAULT_PRESSED_MS),
            unsafe_duration: Some(DEFAULT_UNSAFE_MS),
        }
    }

    fn load_settings() -> Self {
        match file::read_toml::<Self>(GAME_SETTINGS_FILE) {
            Ok(settings) => settings,
            Err(err) => {
                error!("Unable to load settings: {err}");
                Self::new()
            },
        }
    }

    fn extend_settings(self, loaded_settings: Self) -> Self {
        info!("Extending settings {self} with loaded settings: {loaded_settings}");

        let new_settings = Self {
            pressed_duration: loaded_settings.pressed_duration.or(self.pressed_duration),
            unsafe_duration: loaded_settings.unsafe_duration.or(self.unsafe_duration),
        };

        info!("New settings: {new_settings}");
        new_settings
    }

    // Getters:
    // As we build from defaults in `Self::new()` every `self.Option` is guaranteed to be
    // `Some([value])` so unwrap the first `Option` in the getter
    pub fn pressed_duration(&self) -> Duration { Duration::from_millis(self.pressed_duration.unwrap()) }

    pub fn unsafe_duration(&self) -> Duration { Duration::from_millis(self.unsafe_duration.unwrap()) }

    // Setters:
    // Set a value
    pub fn set_pressed_duration(&mut self, millis: u64) { self.pressed_duration = Some(millis); }

    pub fn set_unsafe_duration(&mut self, millis: u64) { self.unsafe_duration = Some(millis); }
}
