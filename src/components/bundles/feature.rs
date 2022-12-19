use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct FeatureBundle {
    pub tag: FeatureTag,
    pub cleanup: CleanupOnExitGame,
}
