use crate::prelude::*;

pub fn update_app_settings(mut app_settings: AppSettings, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() { 
        if window.mode() == WindowMode::Windowed {
            app_settings.set_window_size(Vec2 {
                x: window.width(),
                y: window.height(),
            });
            app_settings.set_fullscreen(false);
        } else {
            app_settings.set_fullscreen(true);
        }
    }
}
