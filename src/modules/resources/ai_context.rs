use crate::prelude::{
    *,
    resources::game_context::*,
    types::random::*,
};

#[derive(Resource)]
pub struct AiContext {
    pub random: Random,
}

impl FromWorld for AiContext {
    fn from_world(world: &mut World) -> Self {
        if let Some(mut game_context) = world.get_resource_mut::<GameContext>() {
            Self {
                random: Random::new(game_context.random.prng.next_u64()),
            }
        } else {
            Self {
                random: Random::from_entropy(),
            }
        }
    }
}