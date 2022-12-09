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

    pub const fn direction(self) -> Option<GridDirection> {
        match self {
            Self::NorthWest => Some(GridDirection::NorthWest),
            Self::North => Some(GridDirection::North),
            Self::NorthEast => Some(GridDirection::NorthEast),
            Self::East => Some(GridDirection::East),
            Self::SouthEast => Some(GridDirection::SouthEast),
            Self::South => Some(GridDirection::South),
            Self::SouthWest => Some(GridDirection::SouthWest),
            Self::West => Some(GridDirection::West),
            _ => None,
        }
    }
}
