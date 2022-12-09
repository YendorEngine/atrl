use crate::prelude::{
    *,
    systems::*,
};

pub enum PathFinder {
    Astar,
    Dijkstras,
}

impl PathFinder {
    pub fn compute(
        &self,
        origin: Position,
        destination: Position,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> Option<Vec<Position>> {
        match self {
            Self::Astar => AStar::compute_path(
                origin,
                destination,
                movement_type,
                partial_path_on_failure,
                provider,
                q_blocks_movement,
            ),
            Self::Dijkstras => Dijkstras::compute_path(
                origin,
                destination,
                movement_type,
                partial_path_on_failure,
                provider,
                q_blocks_movement,
            ),
        }
    }
}
