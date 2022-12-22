use crate::{prelude::*, systems::functions::*};

#[derive(Resource)]
pub struct TestGenConfig {
    pub seed: u64,
    pub show_menu: bool,
    pub stars: Vec<IVec2>,
    pub noise: NoiseConfig,
    pub spiral: SpiralConfig,
    pub selected_type: GeneratorType,
}

impl Default for TestGenConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            show_menu: true,
            stars: Vec::new(),
            noise: NoiseConfig::default(),
            spiral: SpiralConfig::default(),
            selected_type: GeneratorType::Noise,
        }
    }
}

impl TestGenConfig {
    pub fn generate_stars(&mut self) { self.stars = self.selected_type.generate_stars(self); }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GeneratorType {
    #[default]
    Noise,
    Spiral,
}

impl GeneratorType {
    pub fn generate_stars(&self, config: &TestGenConfig) -> Vec<IVec2> {
        info!("GeneratorType: {:?}", self);
        let ret = match self {
            Self::Spiral => generate_spiral(&config.spiral),
            Self::Noise => generate_noise(&config.noise),
        };
        info!("Generated {} points.", ret.len());
        ret
    }
}

// Spiral
pub struct SpiralConfig {
    pub max_stars: u32,
    pub arm_count: u32, // 1 - 6
    pub draw_lines: bool,
}

impl Default for SpiralConfig {
    fn default() -> Self {
        Self {
            arm_count: 6, // 1 - 6
            max_stars: 100,
            draw_lines: true,
        }
    }
}

// Noise
pub struct NoiseConfig {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub cutoff: f64,

    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            cutoff: 0.5,

            octaves: 6,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }
}
