use crate::prelude::*;

#[derive(Component, Reflect, Default, Serialize, Deserialize, Clone, Copy, Deref, DerefMut)]
pub struct ChunkPosition(pub UVec2);
