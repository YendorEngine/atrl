use crate::prelude::*;
use crate::resources::*;

pub fn init_app_settings(
    mut commands: Commands,
) {
    commands.init_resource::<AppSettings>()
}