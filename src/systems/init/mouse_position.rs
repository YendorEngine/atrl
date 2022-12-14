use crate::{prelude::*, resources::*};

pub fn init_mouse_position(mut commands: Commands) { commands.init_resource::<MousePosition>() }
