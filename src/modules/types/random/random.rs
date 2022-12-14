use crate::{modules::types::*, prelude::*};

pub struct Random {
    pub seed: u64,
    pub prng: Pcg64,
    pub prht: Prht,
    pub noise: Fbm<Perlin>,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        let mut prng = Pcg64::seed_from_u64(seed);
        let prht = Prht::new(prng.next_u64());
        let noise = Fbm::new(prng.next_u32());
        Self {
            seed,
            prng,
            prht,
            noise,
        }
    }

    pub fn from_entropy() -> Self { Self::new(Pcg64::from_entropy().next_u64()) }

    pub fn get_prng(&mut self) -> &mut Pcg64 { &mut self.prng }

    pub fn get_prht(&mut self) -> &mut Prht { &mut self.prht }

    pub fn get_noise(&mut self) -> &mut Fbm<Perlin> { &mut self.noise }
}

impl Default for Random {
    fn default() -> Self { Self::from_entropy() }
}

impl std::fmt::Debug for Random {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random({:?})", self.seed)
    }
}
