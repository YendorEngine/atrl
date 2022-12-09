use std::fmt::Display;

use crate::prelude::*;

#[derive(Default, Reflect, FromReflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LocalPosition(UVec3);

impl LocalPosition {
    pub const GRID_SIZED: Self = Self::new(GRID_WIDTH, GRID_HEIGHT, 0);
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    #[inline(always)]
    pub const fn new(x: u32, y: u32, layer: u32) -> Self { Self(UVec3::new(x, y, layer)) }

    /// Creates a `LocalPostion` with `x` and `y` set to `v`. and `layer` set to `0`
    #[inline]
    pub const fn splat(v: u32) -> Self { Self(UVec2::splat(v).extend(0)) }

    ///////////////////////////////
    /// Getters
    ///////////////////////////////
    #[inline]
    pub const fn x(&self) -> u32 { self.0.x }

    #[inline]
    pub const fn y(&self) -> u32 { self.0.y }

    #[inline]
    pub const fn layer(&self) -> u32 { self.0.z }

    #[inline]
    pub const fn gridpoint(&self) -> UVec2 { UVec2::new(self.x(), self.y()) }

    #[inline(always)]
    pub fn grid_index(&self, size: UVec2) -> Option<usize> { self.gridpoint().as_index(size) }

    ///////////////////////////////
    /// Setters
    ///////////////////////////////
    pub fn set_x(&mut self, value: u32) { self.0.x = value; }

    pub fn set_y(&mut self, value: u32) { self.0.y = value; }

    pub fn set_layer(&mut self, value: u32) { self.0.z = value; }

    pub fn set_xy(&mut self, x: u32, y: u32) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn translation(&self) -> Vec3 {
        Vec3::new(
            self.x() as f32 + 0.5,
            self.y() as f32 + 0.5,
            (self.layer() as f32).mul_add(2.0, 1.0),
        )
    }
}

impl Display for LocalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}):{}", self.x(), self.y(), self.layer())
    }
}
