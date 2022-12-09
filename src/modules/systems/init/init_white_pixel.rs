use crate::prelude::*;

pub fn init_white_pixel(
    mut commands: Commands,
) {
    commands.init_resource::<WhitePixel>();
}