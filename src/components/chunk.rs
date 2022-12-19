use crate::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct ChunkComponent(pub Vec<Entity>);