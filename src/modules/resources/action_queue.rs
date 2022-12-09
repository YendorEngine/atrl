use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct ActionQueue {
    actions: VecDeque<BoxedAction>,
}

impl ActionQueue {
    /// Queue an action to perform
    pub fn add_action(&mut self, action: BoxedAction) { self.actions.push_back(action); }

    /// Retrieve an action to perform it (By `TurnManager`)
    pub fn get_action(&mut self) -> Option<BoxedAction> { self.actions.pop_front() }

    /// Clear the actions. Interrupted input because unsafe event.
    pub fn clear_actions(&mut self) { self.actions.clear(); }
}
