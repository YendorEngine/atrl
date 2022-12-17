use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
// could be used for temporary storage for multi turn actions
pub struct AttackActor;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    pub generated_path: bool,
    pub last_seen_pt: Option<ChunkPosition>,
}

// could be used for temporary storage for multi turn actions
#[derive(Debug, Component, Clone, Eq, PartialEq)]
pub struct Wander {
    pub destination: Option<ChunkPosition>,
    pub my_previous_location: ChunkPosition,
}

impl Default for Wander {
    fn default() -> Self {
        Self {
            destination: None,
            my_previous_location: ChunkPosition::ZERO,
        }
    }
}
