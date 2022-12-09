use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct ActionQueue {
    actions: VecDeque<ActionType>,
}

impl ActionQueue {
    /// Queue an action to perform
    pub fn add_action(&mut self, action: ActionType) { self.actions.push_back(action); }

    /// Retrieve an action to perform it (By `TurnManager`)
    pub fn get_action(&mut self) -> Option<ActionType> { self.actions.pop_front() }

    /// Clear the actions. Interrupted input because unsafe event.
    pub fn clear_actions(&mut self) { self.actions.clear(); }
}
