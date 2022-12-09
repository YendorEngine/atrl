use crate::prelude::*;

pub const NUM_DIRECTIONS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum GridDirection {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DirectionType {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

impl GridDirection {
    pub fn from_unit_coord<P>(coord: impl GridPoint + std::fmt::Debug) -> Self {
        match [coord.x(), coord.y()] {
            [1, 0] => Self::East,
            [-1, 0] => Self::West,
            [0, 1] => Self::South,
            [0, -1] => Self::North,
            [1, 1] => Self::SouthEast,
            [1, -1] => Self::NorthEast,
            [-1, 1] => Self::SouthWest,
            [-1, -1] => Self::NorthWest,
            _ => panic!("Unexpected coord: {coord:?}"),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }

    // Bevy Transform uses N: 1, S: -1, E: 1, W: -1
    pub const fn coord(self) -> IVec2 {
        match self {
            Self::North => IVec2::new(0, 1),
            Self::NorthEast => IVec2::new(1, 1),

            Self::East => IVec2::new(1, 0),
            Self::SouthEast => IVec2::new(1, -1),

            Self::South => IVec2::new(0, -1),
            Self::SouthWest => IVec2::new(-1, -1),

            Self::West => IVec2::new(-1, 0),
            Self::NorthWest => IVec2::new(-1, 1),
        }
    }

    pub const fn left90(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::NorthEast => Self::NorthWest,
            Self::East => Self::North,
            Self::SouthEast => Self::NorthEast,
            Self::South => Self::East,
            Self::SouthWest => Self::SouthEast,
            Self::West => Self::South,
            Self::NorthWest => Self::SouthWest,
        }
    }

    pub const fn right90(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::NorthEast => Self::SouthEast,
            Self::East => Self::South,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::West,
            Self::SouthWest => Self::NorthWest,
            Self::West => Self::North,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub const fn left45(self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::NorthEast => Self::North,
            Self::East => Self::NorthEast,
            Self::SouthEast => Self::East,
            Self::South => Self::SouthEast,
            Self::SouthWest => Self::South,
            Self::West => Self::SouthWest,
            Self::NorthWest => Self::West,
        }
    }

    pub const fn right45(self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }

    pub const fn left135(self) -> Self {
        match self {
            Self::North => Self::SouthWest,
            Self::NorthEast => Self::West,
            Self::East => Self::NorthWest,
            Self::SouthEast => Self::North,
            Self::South => Self::NorthEast,
            Self::SouthWest => Self::East,
            Self::West => Self::SouthEast,
            Self::NorthWest => Self::South,
        }
    }

    pub const fn right135(self) -> Self {
        match self {
            Self::North => Self::SouthEast,
            Self::NorthEast => Self::South,
            Self::East => Self::SouthWest,
            Self::SouthEast => Self::West,
            Self::South => Self::NorthWest,
            Self::SouthWest => Self::North,
            Self::West => Self::NorthEast,
            Self::NorthWest => Self::East,
        }
    }

    pub const fn bitmap_raw(self) -> u8 { 1 << self as usize }

    pub const fn bitmap(self) -> DirectionBitmap { DirectionBitmap::new(self.bitmap_raw()) }

    pub const fn is_cardinal(self) -> bool {
        matches!(self, Self::North | Self::East | Self::South | Self::West)
    }

    pub const fn is_ordinal(self) -> bool {
        matches!(
            self,
            Self::NorthEast | Self::SouthEast | Self::SouthWest | Self::NorthWest
        )
    }

    pub const fn typ(self) -> DirectionType {
        match self {
            Self::North => DirectionType::Cardinal(CardinalDirection::North),
            Self::NorthEast => DirectionType::Ordinal(OrdinalDirection::NorthEast),
            Self::East => DirectionType::Cardinal(CardinalDirection::East),
            Self::SouthEast => DirectionType::Ordinal(OrdinalDirection::SouthEast),
            Self::South => DirectionType::Cardinal(CardinalDirection::South),
            Self::SouthWest => DirectionType::Ordinal(OrdinalDirection::SouthWest),
            Self::West => DirectionType::Cardinal(CardinalDirection::West),
            Self::NorthWest => DirectionType::Ordinal(OrdinalDirection::NorthWest),
        }
    }

    pub const fn cardinal(self) -> Option<CardinalDirection> {
        match self {
            Self::North => Some(CardinalDirection::North),
            Self::East => Some(CardinalDirection::East),
            Self::South => Some(CardinalDirection::South),
            Self::West => Some(CardinalDirection::West),
            _ => None,
        }
    }

    pub const fn ordinal(self) -> Option<OrdinalDirection> {
        match self {
            Self::NorthEast => Some(OrdinalDirection::NorthEast),
            Self::SouthEast => Some(OrdinalDirection::SouthEast),
            Self::SouthWest => Some(OrdinalDirection::SouthWest),
            Self::NorthWest => Some(OrdinalDirection::NorthWest),
            _ => None,
        }
    }

    pub const fn all() -> DirectionIter { DirectionIter::new() }
}

impl From<GridDirection> for [i32; 2] {
    fn from(d: GridDirection) -> [i32; 2] {
        use self::GridDirection::*;
        match d {
            North => [0, 1],
            East => [1, 0],
            South => [0, -1],
            West => [-1, 0],
            NorthWest => [-1, 1],
            NorthEast => [1, 1],
            SouthEast => [1, -1],
            SouthWest => [-1, -1],
        }
    }
}

impl From<GridDirection> for (i32, i32) {
    fn from(d: GridDirection) -> (i32, i32) {
        use self::GridDirection::*;
        match d {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
            NorthWest => (-1, 1),
            NorthEast => (1, 1),
            SouthEast => (1, -1),
            SouthWest => (-1, -1),
        }
    }
}

impl FromIterator<GridDirection> for &[GridDirection] {
    fn from_iter<T: IntoIterator<Item = GridDirection>>(iter: T) -> Self {
        let mut v = Vec::new();
        for d in iter {
            v.push(d);
        }
        Box::leak(v.into_boxed_slice())
    }
}

impl Distribution<GridDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridDirection {
        let index = rng.gen_range(0..NUM_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
