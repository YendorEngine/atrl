use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
// could be used for temporary storage for multi turn actions
pub struct AttackActor;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    pub generated_path: bool,
    pub last_seen_pt: Option<Position>,
}

// could be used for temporary storage for multi turn actions
#[derive(Debug, Reflect, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    pub destination: Option<Position>,
    pub my_previous_location: Position,
}

impl Default for Wander {
    fn default() -> Self {
        Self {
            destination: None,
            my_previous_location: Position::new(
                WorldPosition::ZERO,
                LocalPosition::new(0, 0, MapLayer::Actors as u32),
            ),
        }
    }
}
