use crate::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    // Movement
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,

    Wait,

    MouseLeftClick,
    MouseRightClick,
}
impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 8] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
    ];

    pub const fn direction(self) -> Option<Direction> {
        match self {
            Self::West => Some(Direction::WEST),
            Self::East => Some(Direction::EAST),
            Self::North => Some(Direction::NORTH),
            Self::South => Some(Direction::SOUTH),
            Self::NorthEast => Some(Direction::NORTH_EAST),
            Self::SouthEast => Some(Direction::SOUTH_EAST),
            Self::SouthWest => Some(Direction::SOUTH_WEST),
            Self::NorthWest => Some(Direction::NORTH_WEST),
            _ => None,
        }
    }
}
