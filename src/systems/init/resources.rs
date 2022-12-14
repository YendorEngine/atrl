use crate::{prelude::*, resources::*};

pub fn init_resources(mut commands: Commands) {
    commands.init_resource::<GameContext>();
    commands.init_resource::<AiContext>();
}
