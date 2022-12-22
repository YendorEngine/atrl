use crate::{components::*, prelude::*};

#[derive(Default, Bundle)]
pub struct TerrainBundle {
    pub tag: TerrainTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for TerrainBundle {
    fn default() -> Self {
        Self {
            tag: TerrainTag,
            cleanup: CleanupOnExitGame,
        }
    }
}