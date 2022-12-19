use crate::prelude::*;

#[derive(Resource)]
pub struct GameSettingsResource {
    pub chunk_size: UVec2,
    pub seed: u64,
}
