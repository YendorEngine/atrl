use crate::{prelude::*, modules::resources::*};

pub fn init_map_manager(
    mut commands: Commands,
    tilesets: Tilesets,
    mut game_context: ResMut<GameContext>,
) {
    let world_position = WorldPosition::new(0, 0, 0);
    let map = MapManager::internal_create_map(&mut commands, &mut game_context, world_position);
    let (terrain_layer, features_layer) = MapManager::internal_create_tilemaps(&mut commands, &tilesets);
    commands.insert_resource(MapManagerResource::new(
        world_position,
        map,
        terrain_layer,
        features_layer,
    ));
}