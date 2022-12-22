use crate::{
    prelude::*,
    types::generator::{NoiseConfig, TestGenConfig},
};

pub fn init_generator_config(mut commands: Commands, app_settings: AppSettings) {
    let grid_size = app_settings.get_grid_size();
    let offset_x = -(grid_size.x as i32 / 2);
    let offset_y = -(grid_size.y as i32 / 2);

    commands.insert_resource(TestGenConfig {
        noise: NoiseConfig {
            left: offset_x,
            bottom: offset_y,
            top: grid_size.y as i32 + offset_y,
            right: grid_size.x as i32 + offset_x,
            ..default()
        },
        ..default()
    })
}
