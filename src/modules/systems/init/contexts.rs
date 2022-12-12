use crate::{prelude::*, modules::resources::*};

pub fn init_contexts(
    mut commands: Commands,
) {
    commands.init_resource::<GameContext>();
    commands.init_resource::<AiContext>();
}