use crate::prelude::*;

pub fn update_window(app_settings: AppSettings, mut windows: ResMut<Windows>) {
    if app_settings.is_changed() {
        info!("App settings changed, updating window...");

        if let Some(window) = windows.get_primary_mut() {
            let window_mode = app_settings.get_window_mode();
            let window_size = app_settings.get_window_resolution();

            let current_window_mode = window.mode();
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
            if window_mode != current_window_mode {
                info!("Setting window mode to {window_mode:?}");
                window.set_mode(window_mode);
            }
        }
    }
}
