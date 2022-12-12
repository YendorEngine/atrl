use crate::{prelude::*, modules::resources::TurnManager};


pub fn init_turn_manager(
    mut commands: Commands,
) {
    commands.init_resource::<TurnManager>();
}