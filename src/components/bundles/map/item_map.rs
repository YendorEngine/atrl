use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct ItemMapBundle {
    pub tag: ItemTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for ItemMapBundle {
    fn default() -> Self {
        Self {
            tag: ItemTag,
            cleanup: CleanupOnExitGame,
        }
    }
}
