use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct BlocksMovement {
    blocked_movement: u8,
}

impl BlocksMovement {
    pub const fn is_blocked(&self, movement_type: u8) -> bool {
        // if movement_type has nothing set more than blocked movement, it's blocked
        self.blocked_movement | movement_type == self.blocked_movement
    }
}
