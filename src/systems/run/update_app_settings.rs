use bevy::window::WindowResized;

use crate::{prelude::*, types::resolution::*};

pub struct Mode(pub WindowMode);

impl Default for Mode {
    fn default() -> Self { Self(WindowMode::Windowed) }
}

pub fn update_app_settings(
    windows: Res<Windows>,
    mut app_settings: AppSettings,
    mut resize_reader: ResMut<Events<WindowResized>>,
) {
    for WindowResized { width, height, .. } in resize_reader.drain() {
        // if !app_settings.is_changed() {
        //     continue;
        // }

        if let Some(window) = windows.get_primary() {
            let window_mode = app_settings.get_window_mode();
            let current_window_size = app_settings.get_window_resolution().get_dimensions();

            let current_window_mode = window.mode();
            let resized_window_size = Vec2 {
                x: width,
                y: height,
            };

            info!("Current window size: {current_window_size} vs: Resize Event: {resized_window_size}");

            if current_window_mode == WindowMode::Windowed && current_window_size != resized_window_size {
                info!("Setting app_settings window size to {resized_window_size}");
                app_settings.set_window_mode(WindowMode::Windowed);
                app_settings.set_window_resolution(Resolution::from(resized_window_size));
            } else if current_window_mode != WindowMode::Windowed && current_window_mode != window_mode {
                info!("Setting app_settings window_mode to {current_window_mode:?}");
                app_settings.set_window_mode(current_window_mode);
            }
        }
    }
}
