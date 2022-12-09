use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct BlocksVision {
    blocked_vision: u8,
}

impl BlocksVision {
    pub const fn is_blocked(&self, vision_type: u8) -> bool {
        // if vision_type has nothing set more than blocked vision, it's blocked
        self.blocked_vision | vision_type == self.blocked_vision
    }
}
