use crate::prelude::*;

mod tiny_galaxy;
pub use tiny_galaxy::*;

/// Long name
pub const APP_NAME: &str = "Away Team Roguelike";
/// Short name
pub const APP_NAME_SHORT: &str = "ATRL";

pub const CHUNK_SIZE: UVec2 = UVec2::new(16, 9);
