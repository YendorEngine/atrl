use crate::{prelude::*, components::*};

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub position: WorldPositionComponent,
}