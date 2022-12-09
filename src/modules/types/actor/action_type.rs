use crate::prelude::{
    *,
    resources::*,
};

pub const WAIT_TIME: u32 = SECONDS;
pub const MOVE_TIME: u32 = SECONDS * 2;
pub const ATTACK_TIME: u32 = (SECONDS as f32 * 1.5) as u32;

#[derive(Debug, Reflect, FromReflect, Clone, Copy)]
pub enum ActionType {
    Wait,
    Attack(Position),
    Movement(Position),
    MovementDelta(IVec2),
}

impl ActionType {
    pub const fn get_base_time_to_perform(&self) -> u32 {
        match self {
            Self::Wait => WAIT_TIME,
            Self::Attack(_) => ATTACK_TIME,
            Self::Movement(_) => MOVE_TIME,
            Self::MovementDelta(_) => MOVE_TIME,
        }
    }
}

// TODO: Implement:
pub trait Action {
    fn get_base_time_to_perform(&self) -> u32;
    fn perform(&mut self) -> Result<(), Box<dyn Action>>;
}
