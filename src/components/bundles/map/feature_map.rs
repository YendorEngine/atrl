use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct FeatureMapBundle {
    pub tag: FeatureTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for FeatureMapBundle {
    fn default() -> Self {
        Self {
            tag: FeatureTag,
            cleanup: CleanupOnExitGame,
        }
    }
}
