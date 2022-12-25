use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct UiMapBundle {
    pub tag: UiTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for UiMapBundle {
    fn default() -> Self {
        Self {
            tag: UiTag,
            cleanup: CleanupOnExitGame,
        }
    }
}
