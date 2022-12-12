use crate::prelude::*;

pub struct PathPassThroughData<'a, 'w, 's> {
    pub movement_type: u8,
    pub map_manager: MapManager<'w, 's>,
    pub q_blocks_movement: &'a Query<'w, 's, &'a BlocksMovement>,
    // q_features: &'w Query<'w, 's, &Features>,
}

impl<'a, 'w, 's> PathPassThroughData<'a, 'w, 's> {
    pub fn new(
        movement_type: u8,
        map_manager: MapManager<'w, 's>,
        q_blocks_movement: &'a Query<'w, 's, &'a BlocksMovement>,
    ) -> Self {
        Self {
            movement_type,
            map_manager,
            q_blocks_movement,
        }
    }
}
