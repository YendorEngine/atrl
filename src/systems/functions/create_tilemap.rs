use crate::prelude::*;

pub fn spawn_tilemap<B: Bundle>(
    commands: &mut Commands,
    tileset: &Tileset,
    tile_id: u32,
    z_level: f32,
    map_bundle: B,
) -> Entity {
    let entity = commands.spawn(map_bundle).id();
    spawn_tilemap_on_entity(commands, tileset, tile_id, z_level, entity);
    entity
}

pub fn spawn_tilemap_on_entity(
    commands: &mut Commands,
    tileset: &Tileset,
    tile_id: u32,
    z_level: f32,
    entity: Entity,
) {
    let mut tile_storage = TileStorage::empty(GRID_SIZE.into());
    for y in 0..GRID_SIZE.y {
        for x in 0..GRID_SIZE.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands.spawn(
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(entity),
                    texture_index: TileTextureIndex(tile_id),
                    ..Default::default()
                }
            ).id();
            commands.entity(entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = tileset.tile_size();

    let transform = Transform {
        translation: Vec3 {
            x: 0.5,
            y: 0.5,
            z: z_level,
        },
        scale: Vec3 {
            x: 1.0/tile_size.x,
            y: 1.0/tile_size.y,
            z: 1.0,
        },
        ..Default::default()
    };
    commands.entity(entity).insert(
        TilemapBundle {
            tile_size: tile_size.into(),
            grid_size: tile_size.into(),
            size: GRID_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tileset.texture().clone()),
            transform,
            ..Default::default()
        }
    );
}