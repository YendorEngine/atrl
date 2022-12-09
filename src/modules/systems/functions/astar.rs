use crate::prelude::{
    *,
    systems::*,
};

pub struct AStar;

impl PathAlgorithm for AStar {
    fn compute_path(
        origin: Position,
        destination: Position,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> Option<Vec<Position>> {
        // create open/closed lists
        let mut open_nodes = IndexList::new();
        let mut closed_nodes = IndexList::new();

        // add the first node to the open list before starting the loop
        let first_node = AStarNode::new(origin, destination);
        open_nodes.insert_first(first_node);

        // loop through all the nodes
        // return if path is found
        loop {
            if open_nodes.is_empty() {
                break;
            }

            // get the lowest cost node
            if let Some(current_node) = open_nodes.remove_first() {
                if current_node.position() == destination {
                    return Self::reconstruct_path(current_node, &mut closed_nodes);
                }

                // update cardinals
                for cardinal in CardinalDirection::all() {
                    let current_position = current_node.position() + cardinal.coord();
                    current_node.update_at_position(
                        current_position,
                        false,
                        destination,
                        provider,
                        q_blocks_movement,
                        movement_type,
                        &mut open_nodes,
                        &mut closed_nodes,
                    );
                }

                // update ordinals
                for ordinal in OrdinalDirection::all() {
                    let current_position = current_node.position() + ordinal.coord();
                    current_node.update_at_position(
                        current_position,
                        true,
                        destination,
                        provider,
                        q_blocks_movement,
                        movement_type,
                        &mut open_nodes,
                        &mut closed_nodes,
                    );
                }

                // close the current node
                closed_nodes.insert_last(current_node);
            }
        }

        // No path found.
        if partial_path_on_failure {
            let mut index = closed_nodes.first_index();
            let mut best_node_index = index;

            if let Some(best_node) = closed_nodes.get(best_node_index) {
                let mut best_cost = best_node.get_cost_from_end();
                index = closed_nodes.next_index(index);
                while index.is_some() {
                    if let Some(current_node) = closed_nodes.get(index) {
                        let current_cost = current_node.get_cost_from_end();
                        if best_cost > current_cost {
                            best_node_index = index;
                            best_cost = current_cost;
                        }
                    }
                    index = closed_nodes.next_index(index);
                }
            }

            closed_nodes
                .remove(best_node_index)
                .and_then(|best_node| Self::reconstruct_path(best_node, &mut closed_nodes))
        } else {
            None
        }
    }
}

impl AStar {
    /// This will return a path *WITHOUT* the starting point. It also
    /// does not reverse the path, so it will be in the order of last point -> first point.
    fn reconstruct_path(
        finished_node: AStarNode,
        closed_nodes: &mut IndexList<AStarNode>,
    ) -> Option<Vec<Position>> {
        let mut ret = Vec::new();
        let mut current_node = finished_node;
        loop {
            current_node = match current_node.get_from_node() {
                None => {
                    // ret.reverse();
                    return Some(ret);
                },
                Some(position) => {
                    ret.push(current_node.position());
                    match AStarNode::find_node_with_position(closed_nodes, position) {
                        None => return None,
                        Some(index) => closed_nodes.remove(index).unwrap(),
                    }
                },
            }
        }
    }
}
