use std::fmt::Display;

use crate::prelude::*;

pub const NUM_CARDINAL_DIRECTIONS: usize = 4;
pub const ALL_CARDINAL_DIRECTION_BITMAP_RAW: u8 = (1 << GridDirection::North as usize) |
    (1 << GridDirection::East as usize) |
    (1 << GridDirection::South as usize) |
    (1 << GridDirection::West as usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum CardinalDirection {
    North = 0,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn from_unit_coord(coord: impl GridPoint + std::fmt::Debug) -> Self {
        match [coord.x(), coord.y()] {
            [1, 0] => Self::East,
            [-1, 0] => Self::West,
            [0, 1] => Self::South,
            [0, -1] => Self::North,
            _ => panic!("Unexpected coord: {coord:?}"),
        }
    }

    pub fn from_octant(octant: Octant) -> Self {
        // TODO: match on the range??
        match octant.0 {
            0 => Self::East,
            1 => Self::North,
            2 => Self::North,
            3 => Self::West,
            4 => Self::West,
            5 => Self::South,
            6 => Self::South,
            7 => Self::East,
            _ => unreachable!(),
        }
    }

    pub const fn direction(self) -> GridDirection {
        match self {
            Self::North => GridDirection::North,
            Self::East => GridDirection::East,
            Self::South => GridDirection::South,
            Self::West => GridDirection::West,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub const fn coord(self) -> IVec2 {
        match self {
            Self::North => IVec2::new(0, 1),
            Self::East => IVec2::new(1, 0),
            Self::South => IVec2::new(0, -1),
            Self::West => IVec2::new(-1, 0),
        }
    }

    pub const fn left90(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub const fn right90(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub const fn left45(self) -> OrdinalDirection {
        match self {
            Self::North => OrdinalDirection::NorthWest,
            Self::East => OrdinalDirection::NorthEast,
            Self::South => OrdinalDirection::SouthEast,
            Self::West => OrdinalDirection::SouthWest,
        }
    }

    pub const fn right45(self) -> OrdinalDirection {
        match self {
            Self::North => OrdinalDirection::NorthEast,
            Self::East => OrdinalDirection::SouthEast,
            Self::South => OrdinalDirection::SouthWest,
            Self::West => OrdinalDirection::NorthWest,
        }
    }

    pub const fn left135(self) -> OrdinalDirection {
        match self {
            Self::North => OrdinalDirection::SouthWest,
            Self::East => OrdinalDirection::NorthWest,
            Self::South => OrdinalDirection::NorthEast,
            Self::West => OrdinalDirection::SouthEast,
        }
    }

    pub const fn right135(self) -> OrdinalDirection {
        match self {
            Self::North => OrdinalDirection::SouthEast,
            Self::East => OrdinalDirection::SouthWest,
            Self::South => OrdinalDirection::NorthWest,
            Self::West => OrdinalDirection::NorthEast,
        }
    }

    pub const fn axis(self) -> GridAxis {
        match self {
            Self::East | Self::West => GridAxis::X,
            Self::North | Self::South => GridAxis::Y,
        }
    }

    pub const fn sign(self) -> i32 {
        match self {
            Self::South | Self::East => 1,
            Self::North | Self::West => -1,
        }
    }

    pub const fn axis_and_sign(self) -> (GridAxis, i32) {
        match self {
            Self::North => (GridAxis::Y, 1),
            Self::East => (GridAxis::X, 1),
            Self::South => (GridAxis::Y, -1),
            Self::West => (GridAxis::X, -1),
        }
    }

    pub const fn all() -> CardinalDirectionIter { CardinalDirectionIter::new() }

    pub const fn all_directions() -> DirectionCardinalIter { DirectionCardinalIter::new() }

    pub const fn combine(self, other: Self) -> Option<OrdinalDirection> {
        OrdinalDirection::from_cardinals(self, other)
    }
}

impl Display for CardinalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self {
            Self::North => "North",
            Self::East => "East",
            Self::South => "South",
            Self::West => "West",
        };
        write!(f, "{dir}")
    }
}

impl From<CardinalDirection> for [i32; 2] {
    fn from(c: CardinalDirection) -> [i32; 2] {
        use self::CardinalDirection::*;
        match c {
            East => [1, 0],
            South => [0, -1],
            West => [-1, 0],
            North => [0, 1],
        }
    }
}

impl From<CardinalDirection> for (i32, i32) {
    fn from(c: CardinalDirection) -> (i32, i32) {
        use self::CardinalDirection::*;
        match c {
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
            North => (0, 1),
        }
    }
}

impl From<CardinalDirection> for i32 {
    fn from(c: CardinalDirection) -> Self {
        use self::CardinalDirection::*;
        match c {
            East => 0,
            North => 90,
            West => 180,
            South => 270,
        }
    }
}

impl From<i32> for CardinalDirection {
    fn from(i: i32) -> Self {
        let mut i = i;
        loop {
            if i >= 0 {
                break;
            }
            i += 360;
        }

        use self::CardinalDirection::*;
        match i % 360 {
            // loop 360deg back around.
            0..=44 => East,
            45..=134 => North,
            135..=224 => West,
            225..=314 => South,
            315..=359 => East,
            _ => East, // this can't pop as i >= 0 and < 360
        }
    }
}

impl FromIterator<CardinalDirection> for &[CardinalDirection] {
    fn from_iter<T: IntoIterator<Item = CardinalDirection>>(iter: T) -> Self {
        let mut v = Vec::new();
        for d in iter {
            v.push(d);
        }
        Box::leak(v.into_boxed_slice())
    }
}

impl Distribution<CardinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
        let index = rng.gen_range(0..NUM_CARDINAL_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
