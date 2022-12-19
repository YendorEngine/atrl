use crate::{prelude::*, components::*};

#[derive(Bundle)]
pub struct ActorBundle {
    pub tag: ActorTag,
    pub position: WorldPositionComponent,
    pub display: DisplayComponent,
    pub cleanup: CleanupOnEnterMainMenu,
}