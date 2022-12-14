use crate::{prelude::*, types::*};

#[derive(Component)]
pub struct PositionComponent {
    pub position: Position,
    pub layer: MapLayer,
}

impl PositionComponent {
    #[inline]
    pub const fn get(&self) -> Position { self.position }
}
