use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::prelude::*;

pub type BoxedAction = Box<dyn AtrlAction>;

pub trait AtrlAction: Sync + Send + Debug + 'static + DynClone {
    /// Returns the base time it takes to perform the action.
    fn get_base_time_to_perform(&self) -> u32;

    /// Returns the position the action is targeting.
    fn get_target_position(&self) -> Option<Position> { None }

    /// Perform the action. Returns the time it took to perform the action.
    fn perform(&mut self, world: &mut World, entity: Entity) -> Result<u32, BoxedAction>;

    /// Returns the action as a boxed trait object.
    fn boxed(self) -> Box<dyn AtrlAction>
    where Self: std::marker::Sized {
        Box::new(self)
    }
}

dyn_clone::clone_trait_object!(AtrlAction);
