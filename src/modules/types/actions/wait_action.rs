use crate::{modules::types::*, prelude::*};

pub const WAIT_TIME: u32 = SECONDS;

#[derive(Debug, Clone)]
pub struct WaitAction;

impl AtrlAction for WaitAction {
    fn get_base_time_to_perform(&self) -> u32 { WAIT_TIME }

    fn perform(&mut self, _: &mut World, _: Entity) -> Result<u32, BoxedAction> {
        info!("Waiting");
        Ok(self.get_base_time_to_perform())
    }
}
