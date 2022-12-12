use crate::prelude::*;

pub struct MapPathFinder;

// Implement PathProvider
impl<'w, 's> PathProvider<PathPassThroughData<'w, 's>, GRID_SIZE> for MapPathFinder {
    fn get_neighbors(
        &self,
        position: Position,
        pass_through_data: &mut PathPassThroughData<'w, 's>,
    ) -> Vec<Position> {
        let Some(map) = pass_through_data.map_manager.get_map(position.get_world_position()) else { return Vec::new() };
        let mut neighbors = Vec::new();

        for direction in Direction::all() {
            let p = position + direction.coord();
            if map.can_place_actor(
                p.get_local_position(),
                pass_through_data.movement_type,
                pass_through_data.q_blocks_movement,
            ) {
                neighbors.push(p);
            }
        }

        // Example adding 3rd dimension around a stairs feature
        // if let Some(features) = map.get_features(position) {
        // for feature in features {
        // if let Ok(feature) = pass_through_data.q_features.get(feature) {
        // if (feature & Features::StairsUp != 0) | (feature & Features::StairsDown != 0) {
        // for direction in VerticalDirection::all() {
        // let p = position + direction.coord();
        // if let Some(map) = self.get_map(p.get_world_position()) {
        // if map.can_place_actor(p.get_local_position(), pass_through_data.movement_type,
        // pass_through_data.q_blocks_movement) { neighbors.push(p);
        // }
        // }
        // }
        // }
        // }
        // }
        // }

        neighbors
    }
}
