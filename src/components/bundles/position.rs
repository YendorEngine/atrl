use crate::{components::*, prelude::*};

#[derive(Bundle)]
pub struct PositionBundle {
    pub local_position: LocalPositionComponent,
    pub world_position: WorldPositionComponent,
}
