use crate::prelude::*;

pub struct MapPassThroughData {
    pub map_entity: Entity,
    // TODO: Explored tiles should be passed from serialized data for the map on loading, or just a
    // new HashSet pub explored_tiles: HashSet<UVec2>
}

impl From<MapGenData<MapPassThroughData>> for Map {
    fn from(data: MapGenData<MapPassThroughData>) -> Self {
        let mut terrain_types = Grid::new_default(data.size);
        for y in 0..data.size.height() {
            for x in 0..data.size.width() {
                let v = *data.output_grid.get_unchecked((x, y));
                terrain_types.set((x, y), v as usize);
            }
        }

        Self {
            entity: data.user_data.map_entity,
            size: data.size,
            world_position: data.world_position,
            random: data.random,

            update_all: true,
            update_tiles: HashSet::new(),
            // TODO: Add explored_tiles HashSet to MapPassThroughData for serialized data
            explored_tiles: HashSet::new(),

            terrain: terrain_types,
            features: Grid::new_default(data.size),
            actors: Grid::new_default(data.size),
        }
    }
}
