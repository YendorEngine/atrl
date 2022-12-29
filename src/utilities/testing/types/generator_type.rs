use super::{super::systems::functions::*, *};
use crate::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GeneratorType {
    #[default]
    Noise,
    Spiral,
}

impl GeneratorType {
    pub fn generate_stars(&self, config: &TestGenConfig) -> Vec<IVec2> {
        let ret = match self {
            Self::Spiral => generate_spiral(&config.spiral),
            Self::Noise => generate_noise(&config.noise),
        };

        info!(
            "GeneratorType: {:?} - Generated {} points.",
            self,
            ret.len()
        );

        ret
    }
}
