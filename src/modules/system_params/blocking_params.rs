use bevy::ecs::query::QueryEntityError;

use crate::prelude::*;

#[derive(SystemParam)]
pub struct BlockingParams<'w, 's> {
    q_blocks_vision: Query<'w, 's, &'static BlocksVision>,
    q_blocks_movement: Query<'w, 's, &'static BlocksMovement>,
}

impl BlockingParams<'_, '_> {
    #[inline]
    pub const fn get_q_blocks_vision(&self) -> &Query<'_, '_, &'static BlocksVision> { &self.q_blocks_vision }

    #[inline]
    pub fn get_entity_vision(&self, entity: Entity) -> Result<&BlocksVision, QueryEntityError> {
        self.q_blocks_vision.get(entity)
    }

    #[inline]
    pub const fn get_q_blocks_movement(&self) -> &Query<'_, '_, &'static BlocksMovement> {
        &self.q_blocks_movement
    }

    #[inline]
    pub fn get_entity_movement(&self, entity: Entity) -> Result<&BlocksMovement, QueryEntityError> {
        self.q_blocks_movement.get(entity)
    }
}
