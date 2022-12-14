use crate::{prelude::*, resources::*};

pub fn init_turn_manager(mut commands: Commands) { commands.init_resource::<TurnManager>(); }
