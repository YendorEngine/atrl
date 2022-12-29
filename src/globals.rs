use crate::{
    prelude::*,
    resources::universe_generation_settings::UniverseGenerationSettings,
    types::descriptor::{SizeDescriptor, WindowDescriptor},
};

//////////////////////////////////
// App Globals
//////////////////////////////////

pub const APP_NAME: &str = "World Generator";

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };

pub const MIN_SCREEN_SIZE: Vec2 = Vec2 {
    x: 1280.0,
    y: 720.0,
};

//////////////////////////////////
// Game Globals
//////////////////////////////////

pub const DEFAULT_FONT: &str = "julia_mono_regular";

// Sizes are in RENDER_CHUNKs
pub static UNIVERSE_SIZES: Lazy<[SizeDescriptor; 4]> = Lazy::new(|| {
    [
        SizeDescriptor::new("Tiny", UVec2::splat(1)),
        SizeDescriptor::new("Small", UVec2::splat(2)),
        SizeDescriptor::new("Medium", UVec2::splat(4)),
        SizeDescriptor::new("Large", UVec2::splat(8)),
    ]
});

pub static PLANET_SIZES: Lazy<[SizeDescriptor; 6]> = Lazy::new(|| {
    [
        SizeDescriptor::new("Extra Tiny", UVec2::splat(5)),
        SizeDescriptor::new("Tiny", UVec2::splat(15)),
        SizeDescriptor::new("Tiny", UVec2::splat(25)),
        SizeDescriptor::new("Tiny", UVec2::splat(50)),
        SizeDescriptor::new("Tiny", UVec2::splat(75)),
        SizeDescriptor::new("Tiny", UVec2::splat(100)),
    ]
});

pub static WINDOW_DESCRIPTORS: Lazy<[WindowDescriptor; 2]> = Lazy::new(|| {
    [
        WindowDescriptor::new("Windowed", WindowMode::Windowed),
        WindowDescriptor::new("Borderless Fullscreen", WindowMode::BorderlessFullscreen),
    ]
});

//////////////////////////////////
// Game Set Globals
//////////////////////////////////

pub const SYSTEM_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const SECTOR_SIZE: UVec2 = UVec2 { x: 10, y: 10 };
pub static PLANET_SIZE: OnceCell<SizeDescriptor> = OnceCell::new();
pub static UNIVERSE_SIZE: OnceCell<SizeDescriptor> = OnceCell::new();

pub fn set_universe_sizes(universe_generation_settings: &UniverseGenerationSettings) {
    PLANET_SIZE.set(universe_generation_settings.planet_descriptor).unwrap();
    UNIVERSE_SIZE.set(universe_generation_settings.universe_descriptor).unwrap();
}
