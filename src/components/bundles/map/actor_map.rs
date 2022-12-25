use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct ActorMapBundle {
    pub tag: ActorTag,
    pub cleanup: CleanupOnExitGame,
}

impl Default for ActorMapBundle {
    fn default() -> Self {
        Self {
            tag: ActorTag,
            cleanup: CleanupOnExitGame,
        }
    }
}
