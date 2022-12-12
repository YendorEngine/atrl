use crate::{prelude::*, resources::*};

pub const ATTACK_TIME: u32 = SECONDS * 2; // Same as Movement, otherwise, they get another attack after player moves.

#[derive(Debug, Clone)]
pub struct AttackAction(pub Position);

impl AtrlAction for AttackAction {
    fn get_target_position(&self) -> Option<Position> { Some(self.0) }

    fn get_base_time_to_perform(&self) -> u32 { ATTACK_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        match try_attack(entity, self.0, world) {
            Ok(_) => Ok(self.get_base_time_to_perform()),
            Err(a) => Err(a),
        }
    }
}
