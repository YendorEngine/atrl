use super::*;
use crate::prelude::*;

#[derive(Resource, Clone)]
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
    pub fn generate_stars(&mut self) {
        self.stars.clear();
        self.stars = self.selected_type.generate_stars(self);
    }
}
