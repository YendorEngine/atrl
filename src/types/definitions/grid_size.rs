use bevy::{prelude::UVec2, render::once_cell::sync::Lazy};

// FIX / REMOVE ALL THIS :)
pub const GRID_SIZE: UVec2 = UVec2::ZERO;
pub static GRID_WIDTH: Lazy<u32> = Lazy::new(|| GRID_SIZE.x);
pub static GRID_HEIGHT: Lazy<u32> = Lazy::new(|| GRID_SIZE.y);
