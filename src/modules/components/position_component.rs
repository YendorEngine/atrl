use crate::prelude::*;

#[derive(Component)]
pub struct PositionComponent {
    pub position: Position,
    pub layer: MapLayer,
}
