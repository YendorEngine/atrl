use crate::{
    components::{bundles::*, *},
    prelude::*,
    systems::spawn_tilemap,
    types::asset_ids::tilesets::*,
};

pub fn spawn_grid(mut commands: Commands, tilesets: Tilesets, app_settings: AppSettings) {
    let grid_size = app_settings.get_grid_size();

    match tilesets.get_by_id(&TILESET_TINY_GALAXY_WORLD_ID) {
        Some(tileset) => {
            spawn_tilemap(
                &mut commands,
                grid_size,
                tileset,
                TILE_TG_WORLD_FLOOR_TILE_A_ID,
                1.0,
                TerrainBundle {
                    tag: TerrainTag,
                    ..Default::default()
                },
            );
        },
        None => error!("Couldn't find tileset."),
    }

    match tilesets.get_by_id(&TILESET_WHITE_PIXEL_ID) {
        Some(tileset) => {
            spawn_tilemap(
                &mut commands,
                grid_size,
                tileset,
                TILE_WHITE_PIXEL_ID,
                2.0,
                TerrainBundle {
                    tag: TerrainTag,
                    ..Default::default()
                },
            );
        },
        None => error!("Couldn't find tileset."),
    }
}
