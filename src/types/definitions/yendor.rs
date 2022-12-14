use ::yendor::geometry::{
    Circle as YendorCircle, Line as YendorLine, Position as YendorPosition, Rectangle as YendorRectangle,
    Shape as YendorShape,
};

use super::GRID_SIZE;

pub type Line = YendorLine<GRID_SIZE>;
pub trait Shape = YendorShape<GRID_SIZE>;
pub type Circle = YendorCircle<GRID_SIZE>;
pub type Rectangle = YendorRectangle<GRID_SIZE>;
pub type Position = YendorPosition<GRID_SIZE>;
