use crate::prelude::*;
#[derive(Serialize, Deserialize, Clone)]
pub struct Random {
    pub prng: Prng,
    pub prht: Prht,
    pub noise: Noise,
}
#[allow(dead_code)]
impl Random {
    pub fn new(seed: u64) -> Self {
        let mut prng = Prng::new(seed);
        let prht = Prht::new(prng.next_u64());
        let noise = Noise::new(prng.next());
        Self { prng, prht, noise }
    }

    pub fn from_entropy() -> Self { Self::new(Prng::entropy_u64()) }

    pub fn get_prng(&mut self) -> &mut Prng { &mut self.prng }
}
impl Default for Random {
    fn default() -> Self { Self::from_entropy() }
}
impl std::fmt::Debug for Random {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random({:?})", self.prng.seed())
    }
}
