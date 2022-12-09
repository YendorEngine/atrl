use crate::prelude::*;

#[derive(Resource)]
/// DON'T USE THIS!!!
/// USE MapManager instead!!!!
pub struct MapManagerResource {
    pub current_map: (WorldPosition, Map),
    pub loaded_maps: HashMap<WorldPosition, Map>,
    pub visible_tiles: VisibilityMap,
    pub terrain_layer: Entity,
    pub features_layer: Entity,
}

// Constructor
impl MapManagerResource {
    pub fn new(
        world_position: WorldPosition,
        map: Map,
        terrain_layer: Entity,
        features_layer: Entity,
    ) -> Self {
        Self {
            current_map: (world_position, map),
            loaded_maps: HashMap::new(),
            visible_tiles: VisibilityMap::new(),
            terrain_layer,
            features_layer,
        }
    }
}
