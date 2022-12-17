use crate::{prelude::*, resources::*};

pub fn init_resources(mut commands: Commands) {
    commands.init_resource::<AiContext>();
    commands.init_resource::<GameContext>();
    commands.init_resource::<MapManagerResource>();

    commands.init_resource::<WhitePixel>();
    commands.init_resource::<TurnManager>();
    commands.init_resource::<MousePosition>();
}
