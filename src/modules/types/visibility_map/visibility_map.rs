use crate::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct VisibilityMap {
    visible_positions: HashSet<Position>,
}

impl VisibilityMap {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            visible_positions: HashSet::new(),
        }
    }

    pub fn iter(&self) -> bevy::utils::hashbrown::hash_set::Iter<Position> { self.visible_positions.iter() }
}

impl FovReceiver for VisibilityMap {
    fn get_visible(&self, position: Position) -> bool { self.visible_positions.contains(&position) }

    fn set_visible(&mut self, position: Position) { self.visible_positions.insert(position); }

    fn get_all(&self) -> HashSet<Position> { self.visible_positions.clone() }
}

impl IntoIterator for VisibilityMap {
    type IntoIter = bevy::utils::hashbrown::hash_set::IntoIter<Position>;
    type Item = Position;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.visible_positions.into_iter() }
}
