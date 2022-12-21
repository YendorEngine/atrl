use crate::prelude::*;

pub const APP_NAME: &str = "World Generator";
pub const MIN_SCREEN_SIZE: Vec2 = Vec2 {
    x: 1280.0,
    y: 720.0,
};

pub const DEFAULT_FONT: &str = "julia_mono_regular";

pub static RESOLUTIONS: Lazy<[(&str, (f32, f32)); 4]> = Lazy::new(|| {
    [
        ("720p", (1280., 720.)),
        ("1080p", (1920., 1080.)),
        ("1440p", (2560., 1440.)),
        ("2160p", (3840., 2160.)),
    ]
});

pub static WINDOW_MODES: Lazy<[(&str, WindowMode); 3]> = Lazy::new(|| {
    [
        ("Windowed", WindowMode::Windowed),
        ("Fullscreen", WindowMode::Fullscreen),
        ("Borderless Fullscreen", WindowMode::BorderlessFullscreen),
    ]
});
