use crate::prelude::*;

pub trait FovProvider {
    fn is_opaque(
        &mut self,
        position: Position,
        vision_type: u8,
        q_blocks_vision: &Query<&BlocksVision>,
    ) -> bool;
}
