use crate::prelude::*;

pub const APP_NAME: &str = "World Generator";
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

// Sizes are in RENDER_CHUNKs
//pub const UNIVERSE_SIZE: UVec2 = UVec2 { x: INFINITY, y: INFINITY };
pub const SECTOR_SIZE: UVec2 = UVec2 { x: 10, y: 10 };
pub const SYSTEM_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const PLANET_SIZE: &[(UVec2, &'static str)] = &[
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

// TODO: Add ("Custom", (x, y)) or make window fixed, Add ("2160p+", (4096., 2160.))
// use the name portion in the UI not the last dimension.
// probably make a struct? so you can match Vec2 to a name if it exists or "Custom" if it doesn't.
// or possibly make RESOLUTIONS and WINDOW_MODES into a single enum:
// pub enum Resolution {
//   R720,
//   R1080,
//   R1440,
//   R2160,
//   R2160p,
//   Custom(f32, f32),
// }
// Or make it a type
// pub type Resolution = Vec2
// impl Resolution {
//   pub fn get_string(&self) -> String {
//     match self.y {
//       720. => match self.x {
//         1280. => "720p".to_string(),
//         _ => "Custom".to_string(),
//       1080. => match self.x {
// ...
// 
// Adding ("2160p+", (4096., 2160) doesn't change the UI
// Serializing this is kinda meh?
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
        //("Fullscreen", WindowMode::Fullscreen), <--- ewww exclusive mode just use borderless fullscreen instead... I don't know why this is still a thing. Back in the day, you needed to get exclusive access to displays for high performace drawing...
        ("Borderless Fullscreen", WindowMode::BorderlessFullscreen),
    ]
});
