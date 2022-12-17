use crate::prelude::*;

#[derive(Resource, Deref)]
pub struct PlayerEntity(pub Entity);

impl PlayerEntity {
    #[inline(always)]
    pub const fn new(entity: Entity) -> Self { Self(entity) }

    pub const fn current(&self) -> Entity { self.0 }
}
