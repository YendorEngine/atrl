use crate::{
    prelude::*,
    resources::*,
};

pub fn save_settings_on_quit(world: &mut World) {
    if let Some(app_settings) = world.remove_resource::<AppSettingsResource>() {
        app_settings.save(); // consumes app_settings
    }
}
