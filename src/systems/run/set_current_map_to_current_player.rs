use crate::{
    modules::{components::*, system_params::*},
    prelude::*,
    resources::*,
};

pub fn set_current_map_to_current_player(
    mut map_manager: MapManager,
    player_entity: Res<PlayerEntity>,
    q_positions: Query<&PositionComponent>,
) {
    if let Ok(pc) = q_positions.get(player_entity.current()) {
        let position = pc.position;
        if map_manager.get_current_world_position() != position.get_world_position() {
            info!(
                "Switching map to: WorldPosition:{}",
                position.get_world_position()
            );
            map_manager.internal_set_current_map(position.get_world_position());
            let (_world_position, map) = map_manager.get_current_map_mut();
            map.update_all = true;
        }
    }
}
