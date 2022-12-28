use crate::{components::bundles::TerrainBundle, prelude::*, types::map_definitions::GalacticFeature};

pub fn spawn_tilemap<B: Bundle>(
    commands: &mut Commands,
    size: UVec2,
    tileset: &Tileset,
    tile_id: u32,
    z_level: f32,
    map_bundle: B,
) -> Entity {
    let entity = commands.spawn(map_bundle).id();
    spawn_tilemap_on_entity(commands, size, tileset, tile_id, z_level, entity);
    entity
}

pub fn spawn_tilemap_on_entity(
    commands: &mut Commands,
    size: UVec2,
    tileset: &Tileset,
    tile_id: u32,
    z_level: f32,
    entity: Entity,
) {
    let mut tile_storage = TileStorage::empty(size.into());
    for y in 0..size.y {
        for x in 0..size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(entity),
                    texture_index: TileTextureIndex(tile_id),
                    ..Default::default()
                })
                .id();
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
            x: 1.0 / tile_size.x,
            y: 1.0 / tile_size.y,
            z: 1.0,
        },
        ..Default::default()
    };
    commands.entity(entity).insert(TilemapBundle {
        tile_size: tile_size.into(),
        grid_size: tile_size.into(),
        size: size.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(tileset.texture().clone()),
        transform,
        ..Default::default()
    });
}

/// Spawn enough tile chunks to fill the screen.
/// The universe is `infinite`
/// Each tile represents 1 sector
pub fn spawn_universe_tilemap() {}
/// A sector is a step down from the universe, edges lead back to the universe
/// A sector can hold many star systems, many sectors are empty
/// Each tile represents 1 system
pub fn spawn_sector_tilemap() {}
/// A system is a step down from a sector, the edges lead to the next system if possible or
/// back to the universe Many systems are empty, Some may contain derelict ships / graveyards
/// from battles / pirates / etc Each tile represents a "main attraction" -> Planet / asteroid
/// / starbase / military outpost / trade outpost / star / etc
pub fn spawn_system_tilemap() {}
/// Spawns a map for a specified feature: planet / asteroid / starbase / military outpost /
/// trade outpost / star / etc
pub fn spawn_galactic_feature_tilemap<B: Bundle>(
    commands: &mut Commands,
    galactic_feature: GalacticFeature,
) -> Entity {
    spawn_defined_tilemap(
        commands,
        galactic_feature.tile_size,
        &galactic_feature.tileset_texture,
        galactic_feature.tile_ids,
        1.0,
        TerrainBundle::default(),
    )
}

fn spawn_defined_tilemap<B: Bundle>(
    commands: &mut Commands,
    tile_size: Vec2,
    tileset_texture: &Handle<Image>,
    tile_ids: Vec<Vec<u32>>,
    z_level: f32,
    map_bundle: B,
) -> Entity {
    let width = tile_ids.len();
    let height = tile_ids[0].len();
    let mut tile_storage = TileStorage::empty(TilemapSize {
        x: width as u32,
        y: height as u32,
    });

    let map_entity = commands.spawn(map_bundle).id();
    for y in 0..height {
        tile_ids.iter().take(width).for_each(|_id| {});
        for x in 0..width {
            let tile_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(map_entity),
                    texture_index: TileTextureIndex(tile_ids[x][y]),
                    ..Default::default()
                })
                .id();
            commands.entity(map_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // TODO: Center?
    let transform = Transform {
        translation: Vec3 {
            x: 0.5,
            y: 0.5,
            z: z_level,
        },
        scale: Vec3 {
            x: 1.0 / tile_size.x,
            y: 1.0 / tile_size.y,
            z: 1.0,
        },
        ..Default::default()
    };
    commands.entity(map_entity).insert(TilemapBundle {
        tile_size: tile_size.into(),
        grid_size: tile_size.into(),
        size: TilemapSize {
            x: width as u32,
            y: height as u32,
        },
        storage: tile_storage,
        texture: TilemapTexture::Single(tileset_texture.clone()),
        transform,
        ..Default::default()
    });

    map_entity
}
