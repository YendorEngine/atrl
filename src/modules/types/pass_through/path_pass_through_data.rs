use crate::prelude::*;

pub struct PathPassThroughData<'w, 's> {
    pub movement_type: u8,
    pub map_manager: &'w mut MapManager<'w, 's>,
    pub q_blocks_movement: &'w Query<'w, 's, &'static BlocksMovement>,
    // q_features: &'w Query<'w, 's, &Features>,
}
