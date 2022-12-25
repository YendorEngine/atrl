use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct TerrainMapBundle {
    pub tag: TerrainTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for TerrainMapBundle {
    fn default() -> Self {
        Self {
            tag: TerrainTag,
            cleanup: CleanupOnExitGame,
        }
    }
}
