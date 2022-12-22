use crate::{components::*, prelude::*};

#[derive(Default, Bundle)]
pub struct TerrainBundle {
    pub tag: TerrainTag,
    pub display: DisplayComponent,
    pub cleanup: CleanupOnExitGame,
}
