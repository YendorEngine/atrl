use crate::prelude::*;

pub trait FovAlgorithm<'w, 's> {
    fn compute_fov(
        origin: Position,
        vision_type: u8,
        range: u32,
        provider: &mut impl FovProvider,
        q_blocks_vision: &Query<'w, 's, &'static BlocksVision>,
        receiver: &mut impl FovReceiver,
    );
}
