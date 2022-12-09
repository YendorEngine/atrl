use ::pathfinding::prelude::{build_path, dijkstra, dijkstra_partial};

use crate::prelude::*;

pub struct Dijkstras;

impl PathAlgorithm for Dijkstras {
    fn compute_path(
        origin: Position,
        destination: Position,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> Option<Vec<Position>> {
        let dijkstra_path = dijkstra(
            &origin,
            |pt| Self::neighbors(provider, movement_type, pt, q_blocks_movement),
            |pt| *pt == destination,
        )
        .map(|(path, _cost)| path);

        if partial_path_on_failure {
            dijkstra_path.or_else(|| {
                let (paths, _) = dijkstra_partial(
                    &origin,
                    |pt| Self::neighbors(provider, movement_type, pt, q_blocks_movement),
                    |&pt| pt == destination,
                );

                let target = paths
                    .iter()
                    .min_by(|(a_pt, (_, a_cost)), (b_pt, (_, b_cost))| {
                        let a_dist = a_pt.distance(destination);
                        let b_dist = b_pt.distance(destination);
                        (a_dist + *a_cost).cmp(&(b_dist + *b_cost))
                    })
                    .map(|(pt, _)| pt)
                    .unwrap_or(&origin);

                Some(build_path(target, &paths))
            })
        } else {
            dijkstra_path
        }
    }
}

impl Dijkstras {
    pub fn neighbors(
        provider: &mut impl PathProvider,
        movement_type: u8,
        position: &Position,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> arrayvec::IntoIter<(Position, u32), 8> {
        let mut neighbors = ArrayVec::<(Position, u32), 8>::new();

        GridDirection::all().for_each(|dir| {
            let dir_position = *position + dir.coord();
            if provider.is_walkable(dir_position, movement_type, q_blocks_movement) {
                neighbors.push((dir_position, provider.cost(dir_position, movement_type)));
            }
        });

        neighbors.into_iter()
    }
}
