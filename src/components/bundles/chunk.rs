use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub position: WorldPositionComponent,
}
