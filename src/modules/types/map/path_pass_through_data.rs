use crate::prelude::*;

pub struct PathPassThroughData<'a, 'w, 's> {
    pub movement_type: u8,
    pub map_manager: MapManager<'w, 's>,
    pub q_blocks_movement: Query<'w, 's, &'a BlocksMovement>,
    // q_features: &'w Query<'w, 's, &Features>,
}
