use crate::prelude::*;

// Definitions for the displaying of a tile
#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct DisplayComponent{
    pub tile_id: u32,
    pub is_visible: bool,
    //pub color: Color,
    // flip?
}

impl DisplayComponent {
    pub fn new(tile_id: u32) -> Self {
        Self {
            tile_id,
            is_visible: true,
        }
    }
}
