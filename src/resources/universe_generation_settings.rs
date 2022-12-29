use crate::{
    prelude::*,
    utilities::testing::{systems::functions::generate_noise, types::NoiseConfig},
};

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct UniverseGenerationSettings {
    /// TODO: REMOVE THIS ONCE WE HAVE AN ACTUAL BUILDING FLOW FROM GEN SCREEN TO
    /// WORLD SCREEN
    pub stars: Vec<IVec2>,
    pub seed: Option<u64>,
    pub sector_size: UVec2,
    pub system_size: UVec2,
    pub planet_size: (UVec2, &'static str),
    pub universe_size: (UVec2, &'static str),
}

impl Default for UniverseGenerationSettings {
    fn default() -> Self { Self::new() }
}

impl UniverseGenerationSettings {
    pub fn new() -> Self {
        Self {
            seed: None,
            stars: Vec::new(),
            sector_size: SECTOR_SIZE,
            system_size: SYSTEM_SIZE,
            planet_size: PLANET_SIZES[0],
            universe_size: UNIVERSE_SIZES[0],
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            stars: Vec::new(),
            sector_size: SECTOR_SIZE,
            system_size: SYSTEM_SIZE,
            planet_size: PLANET_SIZES[0],
            universe_size: UNIVERSE_SIZES[0],
        }
    }

    pub fn get_universe_display(&self) -> String {
        format!(
            "{size} {width}x{height}",
            width = self.universe_size.0.x,
            height = self.universe_size.0.y,
            size = self.universe_size.1
        )
    }

    pub fn get_planet_display(&self) -> String {
        format!(
            "{size} {width}x{height}",
            width = self.planet_size.0.x,
            height = self.planet_size.0.y,
            size = self.planet_size.1
        )
    }

    pub fn generate_noise(&mut self, grid_size: UVec2) {
        let offset_x = -(grid_size.x as i32 / 2);
        let offset_y = -(grid_size.y as i32 / 2);

        self.stars = generate_noise(&NoiseConfig {
            left: offset_x,
            bottom: offset_y,
            top: grid_size.y as i32 + offset_y,
            right: grid_size.x as i32 + offset_x,
            ..default()
        });
    }
}
