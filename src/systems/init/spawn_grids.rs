use crate::{
    components::{bundles::*, *},
    prelude::*,
    systems::{*, functions::*},
    types::asset_ids::tilesets::*,
};

pub fn spawn_grids(mut commands: Commands, tilesets: Tilesets, app_settings: AppSettings) {
    let grid_size = app_settings.get_grid_size();

    let h_number_of_tilemaps = (grid_size.x as f32 / RENDER_CHUNK_SIZE.x as f32).ceil() as u32;
    let v_number_of_tilemaps = (grid_size.y as f32 / RENDER_CHUNK_SIZE.y as f32).ceil() as u32;

    let terrain_tileset =
        tilesets.get_by_id(&TILESET_TINY_GALAXY_WORLD_ID).expect("Couldn't get terrain tilemap.");
    let terrain_texture = terrain_tileset.texture();

    let feature_tileset =
        tilesets.get_by_id(&TILESET_TINY_GALAXY_WORLD_ID).expect("Couldn't get feater tilemap.");
    let feature_texture = feature_tileset.texture();

    let item_tileset = tilesets.get_by_id(&TILESET_TINY_GALAXY_ITEMS_ID).expect("Couldn't get item tilemap.");
    let item_texture = item_tileset.texture();

    let actor_tileset =
        tilesets.get_by_id(&TILESET_TINY_GALAXY_MONSTERS_ID).expect("Couldn't get actor tilemap.");
    let actor_texture = actor_tileset.texture();

    let ui_tileset = tilesets.get_by_id(&TILESET_TINY_GALAXY_INTERFACE_ID).expect("Couldn't get ui tilemap.");
    let ui_texture = ui_tileset.texture();

    let mut builder = TilemapBuilder::new(render_chunk_size);
    for y in 0..v_number_of_tilemaps {
        for x in 0..h_number_of_tilemaps {
            let position = UVec2::new(x, y);
            builder.set_map_position(position);

            // Spawn Terrain Tilemaps
            let tilemap = builder
                .set_tile_size(terrain_tileset.tile_size())
                .set_tileset_texture(terrain_texture)
                .set_z_level(MapLayer::Terrain)
                .build(&mut commands, TerrainMapBundle::default());
            commands.entity(tilemap).insert(ChunkPosition(position));

            // Spawn Feature Tilemaps
            let tilemap = builder
                .set_tile_size(feature_tileset.tile_size())
                .set_tileset_texture(feature_texture)
                .set_z_level(MapLayer::Features)
                .build(&mut commands, FeatureMapBundle::default());
            commands.entity(tilemap).insert(ChunkPosition(position));

            // Spawn Item Tilemaps
            let tilemap = builder
                .set_tile_size(item_tileset.tile_size())
                .set_tileset_texture(item_texture)
                .set_z_level(MapLayer::Items)
                .build(&mut commands, ItemMapBundle::default());
            commands.entity(tilemap).insert(ChunkPosition(position));

            // Spawn Actor Tilemaps
            let tilemap = builder
                .set_tile_size(actor_tileset.tile_size())
                .set_tileset_texture(actor_texture)
                .set_z_level(MapLayer::Actors)
                .build(&mut commands, ActorMapBundle::default());
            commands.entity(tilemap).insert(ChunkPosition(position));

            // Spawn UI Tilemaps
            let tilemap = builder
                .set_tile_size(ui_tileset.tile_size())
                .set_tileset_texture(ui_texture)
                .set_z_level(MapLayer::Ui)
                .build(&mut commands, UiMapBundle::default());
            commands.entity(tilemap).insert(ChunkPosition(position));
        }
    }
}
