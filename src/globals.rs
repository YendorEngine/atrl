use crate::prelude::*;

pub const APP_NAME: &str = "World Generator";
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

// Sizes are in RENDER_CHUNKs
// pub const UNIVERSE_SIZE: UVec2 = UVec2 { x: INFINITY, y: INFINITY };
pub const SECTOR_SIZE: UVec2 = UVec2 { x: 10, y: 10 };
pub const SYSTEM_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const PLANET_SIZE: &[(UVec2, &str)] = &[
    (UVec2 { x: 100, y: 100 }, "Large"),
    (UVec2 { x: 75, y: 75 }, "Medium Large"),
    (UVec2 { x: 50, y: 50 }, "Medium"),
    (UVec2 { x: 25, y: 25 }, "Small"),
    (UVec2 { x: 15, y: 15 }, "Tiny"),
    (UVec2 { x: 5, y: 5 }, "Extra Tiny"),
];

pub const MIN_SCREEN_SIZE: Vec2 = Vec2 {
    x: 1280.0,
    y: 720.0,
};

pub const DEFAULT_FONT: &str = "julia_mono_regular";

// Serializing this is kinda meh?
pub static RESOLUTIONS: Lazy<[(&str, (f32, f32)); 5]> = Lazy::new(|| {
    [
        ("720p", (1280., 720.)),
        ("1080p", (1920., 1080.)),
        ("1440p", (2560., 1440.)),
        ("2160p", (3840., 2160.)),
        ("2160p+", (4096., 2160.)),
    ]
});

pub static WINDOW_MODES: Lazy<[(&str, WindowMode); 2]> = Lazy::new(|| {
    [
        ("Windowed", WindowMode::Windowed),
        ("Borderless Fullscreen", WindowMode::BorderlessFullscreen),
    ]
});
