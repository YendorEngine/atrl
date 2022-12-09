use crate::prelude::*;

pub trait PathProvider {
    fn is_walkable(
        &mut self,
        position: Position,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool;

    fn cost(&mut self, position: Position, movement_type: u8) -> u32;
}
