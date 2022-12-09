use std::fmt::Display;

use crate::prelude::*;

pub const NUM_ORDINAL_DIRECTIONS: usize = 4;
pub const ALL_ORDINAL_DIRECTION_BITMAP_RAW: u8 = (1 << GridDirection::NorthEast as usize) |
    (1 << GridDirection::SouthEast as usize) |
    (1 << GridDirection::SouthWest as usize) |
    (1 << GridDirection::NorthWest as usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum OrdinalDirection {
    NorthEast = 0,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl OrdinalDirection {
    pub fn from_unit_coord(coord: impl GridPoint + std::fmt::Debug) -> Self {
        match [coord.x(), coord.y()] {
            [1, 1] => Self::SouthEast,
            [1, -1] => Self::NorthEast,
            [-1, 1] => Self::SouthWest,
            [-1, -1] => Self::NorthWest,
            _ => panic!("Unexpected coord: {coord:?}"),
        }
    }

    pub fn from_octant(octant: Octant) -> Self {
        // TODO: match on the range??
        match octant.0 {
            0 => Self::NorthEast,
            1 => Self::NorthEast,
            2 => Self::NorthWest,
            3 => Self::NorthWest,
            4 => Self::SouthWest,
            5 => Self::SouthWest,
            6 => Self::SouthEast,
            7 => Self::SouthEast,
            _ => unreachable!(),
        }
    }

    pub const fn direction(self) -> GridDirection {
        match self {
            Self::NorthEast => GridDirection::NorthEast,
            Self::SouthEast => GridDirection::SouthEast,
            Self::SouthWest => GridDirection::SouthWest,
            Self::NorthWest => GridDirection::NorthWest,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }

    pub const fn coord(self) -> IVec2 {
        match self {
            Self::NorthEast => IVec2::new(1, 1),
            Self::SouthEast => IVec2::new(1, -1),
            Self::SouthWest => IVec2::new(-1, -1),
            Self::NorthWest => IVec2::new(-1, 1),
        }
    }

    pub const fn left90(self) -> Self {
        match self {
            Self::NorthEast => Self::NorthWest,
            Self::SouthEast => Self::NorthEast,
            Self::SouthWest => Self::SouthEast,
            Self::NorthWest => Self::SouthWest,
        }
    }

    pub const fn right90(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub const fn left45(self) -> CardinalDirection {
        match self {
            Self::NorthEast => CardinalDirection::North,
            Self::SouthEast => CardinalDirection::East,
            Self::SouthWest => CardinalDirection::South,
            Self::NorthWest => CardinalDirection::West,
        }
    }

    pub const fn right45(self) -> CardinalDirection {
        match self {
            Self::NorthEast => CardinalDirection::East,
            Self::SouthEast => CardinalDirection::South,
            Self::SouthWest => CardinalDirection::West,
            Self::NorthWest => CardinalDirection::North,
        }
    }

    pub const fn left135(self) -> CardinalDirection {
        match self {
            Self::NorthEast => CardinalDirection::West,
            Self::SouthEast => CardinalDirection::North,
            Self::SouthWest => CardinalDirection::East,
            Self::NorthWest => CardinalDirection::South,
        }
    }

    pub const fn right135(self) -> CardinalDirection {
        match self {
            Self::NorthEast => CardinalDirection::South,
            Self::SouthEast => CardinalDirection::West,
            Self::SouthWest => CardinalDirection::North,
            Self::NorthWest => CardinalDirection::East,
        }
    }

    pub const fn from_cardinals(a: CardinalDirection, b: CardinalDirection) -> Option<Self> {
        match a {
            CardinalDirection::North => match b {
                CardinalDirection::East => Some(Self::NorthEast),
                CardinalDirection::West => Some(Self::NorthWest),
                _ => None,
            },
            CardinalDirection::East => match b {
                CardinalDirection::North => Some(Self::NorthEast),
                CardinalDirection::South => Some(Self::SouthEast),
                _ => None,
            },
            CardinalDirection::South => match b {
                CardinalDirection::East => Some(Self::SouthEast),
                CardinalDirection::West => Some(Self::SouthWest),
                _ => None,
            },
            CardinalDirection::West => match b {
                CardinalDirection::North => Some(Self::NorthWest),
                CardinalDirection::South => Some(Self::SouthWest),
                _ => None,
            },
        }
    }

    pub const fn to_cardinals(self) -> (CardinalDirection, CardinalDirection) {
        use self::{CardinalDirection::*, OrdinalDirection::*};
        match self {
            NorthEast => (North, East),
            SouthEast => (East, South),
            SouthWest => (South, West),
            NorthWest => (West, North),
        }
    }

    pub fn cardinal_bitmap(self) -> DirectionBitmap {
        let (a, b) = self.to_cardinals();
        a.direction().bitmap() | b.direction().bitmap()
    }

    pub const fn all() -> OrdinalDirectionIter { OrdinalDirectionIter::new() }

    pub const fn all_directions() -> DirectionOrdinalIter { DirectionOrdinalIter::new() }
}

impl Display for OrdinalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self {
            Self::NorthEast => "NorthEast",
            Self::SouthEast => "SouthEast",
            Self::SouthWest => "SouthWest",
            Self::NorthWest => "NorthWest",
        };
        write!(f, "{dir}")
    }
}

impl From<OrdinalDirection> for [i32; 2] {
    fn from(o: OrdinalDirection) -> [i32; 2] {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => [-1, 1],
            NorthEast => [1, 1],
            SouthEast => [1, -1],
            SouthWest => [-1, -1],
        }
    }
}

impl From<OrdinalDirection> for (i32, i32) {
    fn from(o: OrdinalDirection) -> (i32, i32) {
        use self::OrdinalDirection::*;
        match o {
            NorthWest => (-1, 1),
            NorthEast => (1, 1),
            SouthEast => (1, -1),
            SouthWest => (-1, -1),
        }
    }
}

impl Distribution<OrdinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OrdinalDirection {
        let index = rng.gen_range(0..NUM_ORDINAL_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
