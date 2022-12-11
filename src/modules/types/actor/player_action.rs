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
            Self::NorthWest => Some(Direction::NorthWest),
            Self::North => Some(Direction::North),
            Self::NorthEast => Some(Direction::NorthEast),
            Self::East => Some(Direction::East),
            Self::SouthEast => Some(Direction::SouthEast),
            Self::South => Some(Direction::South),
            Self::SouthWest => Some(Direction::SouthWest),
            Self::West => Some(Direction::West),
            _ => None,
        }
    }
}
