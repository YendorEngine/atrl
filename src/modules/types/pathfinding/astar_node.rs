use crate::prelude::*;

const SCALE_F32_TO_U32: f32 = 10.0;
const CARDINAL_COST_F32: f32 = 1.0;
const ORDINAL_COST_F32: f32 = 1.4;
const CARDINAL_COST: u32 = (CARDINAL_COST_F32 * SCALE_F32_TO_U32) as u32;
const ORDINAL_COST: u32 = (ORDINAL_COST_F32 * SCALE_F32_TO_U32) as u32;

#[derive(Debug)]
pub struct AStarNode {
    is_walkable: bool,
    position: Position,
    cost_multiplier: u32,
    from_node: Option<Position>,

    cost_from_start: u32,
    cost_from_end: u32,
    cost_total: u32,
}

impl AStarNode {
    pub fn new(origin: Position, destination: Position) -> Self {
        let from_end = origin.distance(destination);
        Self {
            is_walkable: true,
            position: origin,
            cost_multiplier: 0, // we are already here
            from_node: None,

            cost_from_start: u32::MIN,
            cost_from_end: from_end,
            cost_total: from_end,
        }
    }

    fn create_neighbor(
        &self,
        position: Position,
        is_diagonal: bool,
        destination: Position,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
        movement_type: u8,
    ) -> Self {
        let cost_from_end = position.distance(destination);
        let mut s = Self {
            is_walkable: provider.is_walkable(position, movement_type, q_blocks_movement),
            position,
            cost_multiplier: provider.cost(position, movement_type),

            from_node: None,
            cost_from_start: u32::MAX,
            cost_from_end,
            cost_total: u32::MAX,
        };
        if s.is_walkable {
            let new_cost_from_start = self.cost_from_start +
                if is_diagonal { ORDINAL_COST } else { CARDINAL_COST } * s.cost_multiplier;
            s.update_node(self, new_cost_from_start);
        }
        s
    }

    pub const fn position(&self) -> Position { self.position }

    pub const fn get_from_node(&self) -> Option<Position> { self.from_node }

    pub const fn get_cost_from_end(&self) -> u32 { self.cost_from_end }

    fn update_total(&mut self) {
        if self.is_walkable {
            self.cost_total = self.cost_from_start + self.cost_from_end;
        }
    }

    /// perform walkable / cost checks before calling this
    /// this function is "unchecked"
    fn update_node(&mut self, other: &Self, new_cost_from_start: u32) {
        self.cost_from_start = new_cost_from_start;
        self.from_node = Some(other.position);
        self.update_total();
    }

    pub fn update_at_position(
        &self,
        position: Position,
        is_diagonal: bool,
        destination: Position,
        provider: &mut impl PathProvider,
        q_blocks_movement: &Query<&BlocksMovement>,
        movement_type: u8,
        open_nodes: &mut IndexList<Self>,
        closed_nodes: &mut IndexList<Self>,
    ) {
        Self::find_node_with_position(closed_nodes, position).map_or_else(
            || {
                if let Some(neighbor_index) = Self::find_node_with_position(open_nodes, position) {
                    // Update Neighbor
                    let neighbor = open_nodes.get(neighbor_index).unwrap(); // unwrap is safe because we still have a valid index
                    let new_cost_from_start = self.cost_from_start +
                        if is_diagonal { ORDINAL_COST } else { CARDINAL_COST } * neighbor.cost_multiplier;
                    if neighbor.is_walkable && neighbor.cost_from_start > new_cost_from_start {
                        let mut neighbor = open_nodes.remove(neighbor_index).unwrap(); // unwrap is safe because we sill have a valid index
                        neighbor.update_node(self, new_cost_from_start);
                        Self::insert_ordered(open_nodes, neighbor);
                    }
                } else {
                    let new_neighbor = self.create_neighbor(
                        position,
                        is_diagonal,
                        destination,
                        provider,
                        q_blocks_movement,
                        movement_type,
                    );
                    if new_neighbor.is_walkable {
                        Self::insert_ordered(open_nodes, new_neighbor);
                    }
                }
            },
            |_neighbor_index| {
                // Neighbor Closed Nothing to do
            },
        )
    }

    fn insert_ordered(list: &mut IndexList<Self>, node_to_insert: Self) {
        let mut iter_index = list.first_index();
        let mut found_index = None;
        while iter_index.is_some() {
            if let Some(current_node) = list.get(iter_index) {
                if node_to_insert > *current_node {
                    iter_index = list.next_index(iter_index);
                    continue;
                }
                found_index = Some(iter_index);
                break;
            }
            iter_index = list.next_index(iter_index);
        }
        match found_index {
            Some(next_index) => list.insert_before(next_index, node_to_insert),
            None => list.insert_last(node_to_insert),
        };
    }

    pub fn find_node_with_position(list: &IndexList<Self>, position: Position) -> Option<Index> {
        let mut index = list.first_index();
        while index.is_some() {
            if let Some(node) = list.get(index) {
                if position == node.position {
                    return Some(index);
                }
            }
            index = list.next_index(index);
        }
        None
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool { self.position == other.position }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // we use this to insert values into a specific list
        // so we want this as optimized as possible for what
        // we assume that list will look like.
        //
        // most costs we assume will be larger than the front
        // out of 8 possible values being changed around the node
        // in general: 3 of them *may* be good, and the others will be bad
        // so we check `greater` first
        //
        // most costs will be different that the rest, so we should check
        // `less` next. and if that's not valid move to the tie breakers.
        // there are very few ties, but we want to work with the closest tie
        // breaker to the goal first.
        if self.cost_total > other.cost_total {
            Some(std::cmp::Ordering::Greater)
        } else if self.cost_total < other.cost_total || self.cost_from_end < other.cost_from_end {
            Some(std::cmp::Ordering::Less)
        } else if self.cost_from_end > other.cost_from_end {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
