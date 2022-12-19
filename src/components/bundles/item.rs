use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct ItemBundle {
    pub tag: ItemTag,
    pub cleanup: CleanupOnExitGame,
}
