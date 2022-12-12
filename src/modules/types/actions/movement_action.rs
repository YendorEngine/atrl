use crate::{prelude::*, resources::*};

pub const MOVE_TIME: u32 = SECONDS * 2;

#[derive(Debug, Clone)]
pub struct MovementAction(pub Position);

impl AtrlAction for MovementAction {
    fn get_base_time_to_perform(&self) -> u32 { MOVE_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        let destination = self.0;
        match try_move(world, entity, destination) {
            Ok(_) => {
                info!("Movement({})", destination);
                Ok(self.get_base_time_to_perform())
            },
            Err(a) => Err(a),
        }
    }

    fn get_target_position(&self) -> Option<Position> { Some(self.0) }
}

#[derive(Debug, Clone)]
pub struct MovementDeltaAction(pub IVec2);

impl AtrlAction for MovementDeltaAction {
    fn get_base_time_to_perform(&self) -> u32 { MOVE_TIME }

    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction> {
        let mut position_q = world.query::<&mut Position>();
        position_q.get(world, entity).map_or(Err(WaitAction.boxed()), |entity_position| {
            let delta = self.0;
            info!("MovementDelta({:?}) from {}", delta, entity_position);
            Err(MovementAction(*entity_position + delta).boxed())
        })
    }
}
