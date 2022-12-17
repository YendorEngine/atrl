use crate::{prelude::*, resources::*};

pub fn create_tilemap_parent(commands: &mut Commands, name: &str) -> Entity {
    commands.spawn((Name::new(name.to_string()), SpatialBundle::default())).id()
}

pub fn create_tilemap(
    commands: &mut Commands,
    tilesets: &Tilesets,
    app_settings: &AppSettings,
    z_level: f32,
) -> Entity {
    let grid_size = app_settings.get_grid_size();
    let cell_size = app_settings.get_cell_size();

    let tilemap_size = TilemapSize {
        x: grid_size.x,
        y: grid_size.y,
    };

    let tile_size = TilemapTileSize {
        x: cell_size.x as f32,
        y: cell_size.y as f32,
    };

    let map_entity = commands.spawn_empty().id();
    let tile_scale = 1.0;
    let Some(tileset) = tilesets.get_by_id(TINY_GALAXY_ID) else {
        panic!("Couldn't find tileset: {TINY_GALAXY_ID}");
    };

    let tile_id = if let Some(tile) = tileset.select_tile(TG_WORLD_CHAIR) { *tile.0.base_index() } else { 0 };

    let mut tile_storage = TileStorage::empty(tilemap_size);
    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    visible: TileVisible(true),
                    tilemap_id: TilemapId(map_entity),
                    texture_index: TileTextureIndex(tile_id as u32),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(map_entity).insert(TilemapBundle {
        tile_size,
        size: tilemap_size,
        storage: tile_storage,
        grid_size: tile_size.into(),
        map_type: TilemapType::Square,
        texture: TilemapTexture::Single(tileset.texture().clone()),
        transform: Transform {
            translation: Vec3 {
                x: 0.5, // each square is 1.0
                y: 0.5, // center of tile shifts 0.5
                z: z_level,
            },
            scale: Vec3 {
                x: tile_scale / tile_size.x,
                y: tile_scale / tile_size.y,
                z: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });

    map_entity
}

pub fn create_tilemap_on_entity<ZLevel: Into<f32>>(
    commands: &mut Commands,
    entity: Entity,
    dimensions: UVec2,
    z_level: ZLevel,
    tileset: &Tileset,
    tile_scale: f32,
) {
    let tilemap_size = TilemapSize {
        x: dimensions.x,
        y: dimensions.y,
    };

    let tile_size = TilemapTileSize {
        x: tileset.tile_size().x,
        y: tileset.tile_size().y,
    };

    let mut tile_storage = TileStorage::empty(tilemap_size);
    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    visible: TileVisible(false),
                    tilemap_id: TilemapId(entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(entity).insert(TilemapBundle {
        tile_size,
        size: tilemap_size,
        storage: tile_storage,
        grid_size: tile_size.into(),
        map_type: TilemapType::Square,
        texture: TilemapTexture::Single(tileset.texture().clone()),
        transform: Transform {
            translation: Vec3 {
                x: 0.5,
                y: 0.5,
                z: z_level.into(),
            },
            scale: Vec3 {
                x: tile_scale / tile_size.x,
                y: tile_scale / tile_size.y,
                z: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
