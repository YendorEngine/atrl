use crate::{components::TerrainTag, prelude::*, systems::*, utilities::testing::types::*};

#[allow(clippy::too_many_arguments)]
pub fn update_tilemap(
    app_settings: AppSettings,
    config: Res<TestGenConfig>,

    _use_color_map: Local<bool>,
    q_tile_storage: Query<&TileStorage>,
    // mut q_tile_color: Query<&mut TileColor>,
    mut q_tile_ids: Query<&mut TileTextureIndex>,
    // mut q_tile_visibility: Query<&mut TileVisible>,
    q_terrain_map: Query<Entity, With<TerrainTag>>,
) {
    let grid_size = app_settings.get_grid_size();
    let offset_x = -(grid_size.x as i32 / 2);
    let offset_y = -(grid_size.y as i32 / 2);

    for map_entity in q_terrain_map.iter() {
        let Ok(storage) = q_tile_storage.get(map_entity) else { continue; };
        for y in 0..grid_size.y {
            for x in 0..grid_size.x {
                let tile_pos = TilePos { x, y };
                let Some(tile_entity) = storage.checked_get(&tile_pos) else { continue; };
                let Ok(mut texture_index) = q_tile_ids.get_mut(tile_entity) else { continue; };

                let xy = IVec2 {
                    x: x as i32 + offset_x,
                    y: y as i32 + offset_y,
                };

                if config.stars.contains(&xy) {
                    texture_index.0 = STAR_ID;
                } else {
                    texture_index.0 = EMPTY_ID;
                }

                if x as i32 == offset_x && y as i32 == offset_y {
                    texture_index.0 = CENTER_ID;
                }
            }
        }
    }
}
