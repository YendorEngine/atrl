use crate::prelude::*;

pub trait PathAlgorithm {
    fn compute_path(
        origin: Position,
        destination: Position,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> Option<Vec<Position>>;
}
