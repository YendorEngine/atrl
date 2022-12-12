use crate::prelude::*;

pub struct PathPassThroughData<'a, 'w, 's> {
    pub movement_type: u8,
    pub map_manager: &'a mut MapManager<'w, 's>,
    pub q_blocks_movement: &'a Query<'a, 'a, &'static BlocksMovement>,
    // q_features: &'w Query<'w, 's, &Features>,
}
