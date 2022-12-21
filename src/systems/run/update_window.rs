use crate::prelude::*;

pub fn update_window(
    app_settings: AppSettings,
    mut windows: ResMut<Windows>,
    mut cached_full_screen: Local<bool>,
) {
    if app_settings.is_changed() {
        info!("App settings changed, updating window...");

        if let Some(window) = windows.get_primary_mut() {
            let full_screen = app_settings.get_fullscreen();
            let window_size = app_settings.get_window_resolution();

            let current_window_size = Vec2 {
                x: window.width(),
                y: window.height(),
            };

            info!("Current window size: {current_window_size} vs: App Window Settings: {window_size}");

            // Window Size
            if window_size != current_window_size {
                info!("Setting window size to {window_size}");
                window.set_resolution(window_size.x, window_size.y);
            }

            // Full Screen
            if full_screen && !*cached_full_screen {
                info!("Setting window to full screen");
                window.set_mode(WindowMode::Fullscreen);
                *cached_full_screen = true;
            } else if !full_screen && *cached_full_screen {
                info!("Setting window to windowed");
                window.set_mode(WindowMode::Windowed);
                *cached_full_screen = false;
            }
        }
    }
}
