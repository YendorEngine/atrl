use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::prelude::*;

pub const NO_DIRECTIONS_BITMAP_RAW: u8 = 0;
pub const ALL_DIRECTIONS_BITMAP_RAW: u8 = 0xff;
pub const ALL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_DIRECTIONS_BITMAP_RAW,
};

pub const NO_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: NO_DIRECTIONS_BITMAP_RAW,
};

pub const ALL_CARDINAL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_CARDINAL_DIRECTION_BITMAP_RAW,
};

pub const ALL_ORDINAL_DIRECTIONS_BITMAP: DirectionBitmap = DirectionBitmap {
    raw: ALL_ORDINAL_DIRECTION_BITMAP_RAW,
};
/// Set of directions implemented as a bitmap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DirectionBitmap {
    pub raw: u8,
}

impl DirectionBitmap {
    pub const fn new(raw: u8) -> Self { Self { raw } }

    pub const fn empty() -> Self { NO_DIRECTIONS_BITMAP }

    pub const fn all() -> Self { ALL_DIRECTIONS_BITMAP }

    pub const fn all_cardinal() -> Self { ALL_CARDINAL_DIRECTIONS_BITMAP }

    pub const fn all_ordinal() -> Self { ALL_ORDINAL_DIRECTIONS_BITMAP }

    pub const fn has(self, direction: Direction) -> bool { self.raw & (1 << direction as usize) != 0 }

    pub const fn is_empty(self) -> bool { self.raw == NO_DIRECTIONS_BITMAP_RAW }

    pub const fn is_full(self) -> bool { self.raw == ALL_DIRECTIONS_BITMAP_RAW }

    pub const fn and(self, rhs: Self) -> Self { Self::new(self.raw & rhs.raw) }

    pub const fn or(self, rhs: Self) -> Self { Self::new(self.raw | rhs.raw) }
}

impl Default for DirectionBitmap {
    fn default() -> Self { Self::empty() }
}

impl BitOr for DirectionBitmap {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self { self.or(rhs) }
}

impl BitOrAssign for DirectionBitmap {
    fn bitor_assign(&mut self, rhs: Self) { self.raw |= rhs.raw; }
}

impl BitAnd for DirectionBitmap {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self { self.and(rhs) }
}

impl BitAndAssign for DirectionBitmap {
    fn bitand_assign(&mut self, rhs: Self) { self.raw &= rhs.raw; }
}
