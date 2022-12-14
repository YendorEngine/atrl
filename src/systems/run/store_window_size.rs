use crate::{prelude::*, resources::*};

// TODO: Move to an systems::quit::save_settings()
// https://github.com/bevyengine/bevy/issues/6323
// NOTE: hacking in Option<> on each is easier than ordering systems...
pub fn store_window_size(app_settings: Option<ResMut<AppSettingsResource>>, windows: Option<Res<Windows>>) {
    if let Some(mut app_settings) = app_settings {
        if let Some(windows) = windows {
            if let Some(window) = windows.get_primary() {
                if window.mode() == WindowMode::Windowed {
                    app_settings.set_window_size(UVec2 {
                        x: window.width() as u32,
                        y: window.height() as u32,
                    });
                }
                // don't store fullscreen as we can't switch right now anyways...
            }
        }
    }
}
