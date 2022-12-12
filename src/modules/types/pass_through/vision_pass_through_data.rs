use crate::prelude::*;

pub struct VisionPassThroughData<'w, 's> {
    pub vision_type: u8,
    pub q_blocks_vision: &'w Query<'w, 's, &'static BlocksVision>,
}
