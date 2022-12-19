use crate::{prelude::*, components::*};

#[derive(Bundle)]
pub struct TerrainBundle {
    pub tag: TerrainTag,
    pub display: DisplayComponent,
    pub cleanup: CleanupOnEnterMainMenu,
}