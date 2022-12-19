use crate::{prelude::*, components::*};

#[derive(Bundle)]
pub struct ItemBundle {
    pub tag: ItemTag,
    pub cleanup: CleanupOnEnterMainMenu,
}