use crate::prelude::{
    *,
    types::random::*,
};

#[derive(Resource)]
pub struct GameContext {
    pub random: Random,
}

impl Default for GameContext {
    fn default() -> Self {
        let random = Random::from_entropy();
        Self { random }
    }
}
