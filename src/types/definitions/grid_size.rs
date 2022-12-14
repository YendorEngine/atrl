use bevy::prelude::UVec2;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GRID_SIZE: UVec2 = UVec2::ZERO;
    pub static ref GRID_WIDTH: u32 = GRID_SIZE.x;
    pub static ref GRID_HEIGHT: u32 = GRID_SIZE.y;
}

pub const fn set_grid_size(size: UVec2) { *GRID_SIZE = size; }
