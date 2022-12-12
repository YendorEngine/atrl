use crate::prelude::*;

pub struct VisionPassThroughData<'a> {
    pub vision_type: u8,
    pub q_blocks_vision: &'a Query<'static, 'static, &'static BlocksVision>,
}
