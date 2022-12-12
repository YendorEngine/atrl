use crate::prelude::*;
use crate::modules::resources::ui::*;

pub fn init_mouse_position(
    mut commands: Commands,
) {
    commands.init_resource::<MousePosition>()
}