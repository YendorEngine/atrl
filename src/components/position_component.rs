use crate::{prelude::*, types::*};

#[derive(Component, Clone, Copy)]
pub struct PositionComponent {
    layer: MapLayer,
    position: ChunkPosition,
}

impl PositionComponent {
    pub fn new(position: ChunkPosition, layer: MapLayer) -> Self { Self { position, layer } }
}

impl PositionComponent {
    pub fn get_position(&self) -> ChunkPosition { self.position }

    pub fn get_layer(&self) -> MapLayer { self.layer }

    pub fn set_position(&mut self, position: ChunkPosition) { self.position = position; }

    pub fn set_layer(&mut self, layer: MapLayer) { self.layer = layer; }
}

impl Display for PositionComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "PositionComponent {{ position: {} {:?} }}",
            self.position, self.layer,
        )
    }
}
