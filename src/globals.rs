use crate::{prelude::*, types::*};

pub const APP_NAME: &str = "World Generator";
pub const MIN_SCREEN_SIZE: Vec2 = Vec2 {
    x: 1280.0,
    y: 720.0,
};

pub static RESOLUTIONS: Lazy<[Resolution; 4]> = Lazy::new(|| {
    [
        Resolution::new("720p", 1280., 720.),
        Resolution::new("1080p", 1920., 1080.),
        Resolution::new("1440p", 2560., 1440.),
        Resolution::new("2160p", 3840., 2160.),
    ]
});

pub static WINDOW_MODES: Lazy<[(&str, WindowMode); 3]> = Lazy::new(|| {
    [
        ("Windowed", WindowMode::Windowed),
        ("Fullscreen", WindowMode::Fullscreen),
        ("Borderless Fullscreen", WindowMode::BorderlessFullscreen),
    ]
});
