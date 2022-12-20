use bevy::window::WindowResized;

use crate::prelude::*;

pub fn update_app_settings(
    windows: Res<Windows>,
    mut app_settings: AppSettings,
    mut cached_full_screen: Local<bool>,
    mut resize_reader: ResMut<Events<WindowResized>>,
) {
    for WindowResized { width, height, .. } in resize_reader.drain() {
        if !app_settings.is_changed() {
            continue;
        }

        if let Some(window) = windows.get_primary() {
            let full_screen = app_settings.get_fullscreen();
            let current_window_size = app_settings.get_window_size();

            let resized_window_size = Vec2 {
                x: width,
                y: height,
            };

            info!("Current window size: {current_window_size} vs: Resize Event: {resized_window_size}");

            if window.mode() == WindowMode::Windowed && current_window_size != resized_window_size {
                info!("Setting window size to {resized_window_size}\n");
                *cached_full_screen = false;
                app_settings.set_fullscreen(false);
                app_settings.set_window_size(resized_window_size);
            } else if full_screen != *cached_full_screen {
                info!("Setting window to full screen");
                *cached_full_screen = true;
                app_settings.set_fullscreen(true);
            }
        }
    }
}
