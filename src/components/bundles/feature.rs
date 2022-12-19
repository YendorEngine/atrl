use crate::{prelude::*, components::*};

#[derive(Bundle)]
pub struct FeatureBundle {
    pub tag: FeatureTag,
    pub cleanup: CleanupOnEnterMainMenu,
}