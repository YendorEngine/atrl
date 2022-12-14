use crate::{components::*, prelude::*};

pub fn update_tilemaps(
    mut map_manager: MapManager,
    q_storage: Query<&TileStorage>,
    mut q_tiles: Query<(&mut TileTextureIndex, &mut TileVisible, &mut TileColor)>,
    // TODO: Component for holding image index on features???
    mut q_visibility: ParamSet<(
        Query<&mut Visibility>,
        Query<(&mut Visibility, &PositionComponent), With<Mob>>,
    )>,
) {
    // Get storages
    let Ok(terrain_storage) = q_storage.get(map_manager.get_terrain_layer()) else {
        error!("No terrain storage!");
        return;
    };

    let Ok(feature_storage) = q_storage.get(map_manager.get_features_layer()) else {
        error!("No feature storage!");
        return;
    };

    let mut check_next = HashSet::new();
    let (world_position, map) = map_manager.get_current_map_mut();

    if map.update_all {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let tile_pos = TilePos::new(x, y);

                // Update Terrain
                let Some(entity) = terrain_storage.get(&tile_pos) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Ok((mut tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let index = *map.terrain.get_unchecked(UVec2::new(x, y)) as u32;
                tile_texture_index.0 = index;

                // Update Features
                let Some(entity) = feature_storage.get(&tile_pos) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Ok((mut _tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Some(list) = map.features.get_unchecked(UVec2::new(x, y)) else {
                    continue;
                };
                let Some(_feature_entity) = list.first() else {
                    continue;
                };
                // Get feature TileId
                // tile_texture_index.0 = feature_tile_id
            }
        }

        map.update_tiles = check_next;
        map.update_all = false;
    } else {
        let mut points = std::mem::take(&mut map.update_tiles);
        for point in points.drain() {
            let tile_pos = TilePos::new(point.x, point.y);

            // Update Terrain
            let Some(entity) = terrain_storage.get(&tile_pos) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Ok((mut tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let index = *map.terrain.get_unchecked(UVec2::new(point.x, point.y)) as u32;
            tile_texture_index.0 = index;

            // Update Features
            let Some(entity) = feature_storage.get(&tile_pos) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Ok((mut _tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Some(list) = map.features.get_unchecked(UVec2::new(point.x, point.y)) else {
                continue;
            };
            let Some(_feature_entity) = list.first() else {
                continue;
            };
            // Get feature TileId
            // tile_texture_index.0 = feature_tile_id
        }
        map.update_tiles = check_next;
    }

    let mut position = Position::new(*world_position, LocalPosition::new(0, 0));

    // let visible_tiles = map_manager.map_manager.visible_tiles;
    let visible_tiles = map_manager.get_visible_tiles();
    // refresh mutable reference for borrow checker...
    let (current_world_position, map) = map_manager.get_current_map();

    for y in 0..GRID_HEIGHT {
        position.set_y(y);
        for x in 0..GRID_WIDTH {
            position.set_x(x);

            let tile_pos = TilePos::new(x, y);
            let is_explored = map.explored_tiles.contains(&UVec2::new(x, y));

            // Terrain
            if let Some(entity) = terrain_storage.get(&tile_pos) {
                if let Ok(mut visibility) = q_visibility.p0().get_mut(entity) {
                    visibility.is_visible = is_explored;
                }

                if let Ok((_index, mut tile_visibility, mut tile_color)) = q_tiles.get_mut(entity) {
                    tile_visibility.0 = is_explored;
                    tile_color.0.set_a(0.15);
                    if visible_tiles.contains(&position) {
                        tile_color.0.set_a(1.0);
                    }
                }
            }

            // Features
            if let Some(entity) = feature_storage.get(&tile_pos) {
                if let Ok(mut visibility) = q_visibility.p0().get_mut(entity) {
                    visibility.is_visible = map.explored_tiles.contains(&UVec2::new(x, y));
                }
            }
            // tiles too
        }
    }

    // Actors
    for (mut visibility, pc) in q_visibility.p1().iter_mut() {
        let position = pc.position;
        if position.get_world_position() == *current_world_position {
            visibility.is_visible = visible_tiles.contains(&position);
        } else {
            visibility.is_visible = false;
        }
    }
}
