use crate::prelude::*;

pub struct PathPassThroughData<'w, 's> {
    pub movement_type: u8,
    pub q_blocks_movement: &'w Query<'w, 's, &'s BlocksMovement>,
    // q_features: &'w Query<'w, 's, &Features>,
}
