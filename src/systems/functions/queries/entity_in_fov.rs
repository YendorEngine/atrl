use crate::{prelude::*, types::*};

pub fn entity_in_fov<Range: Into<u32>>(
    map_manager: &mut MapManager,
    range: Range,
    vision_type: u8,
    current_pos: ChunkPosition,
    target_pos: ChunkPosition,
) -> bool {
    // // If the player is within the FOV range of the AI, check line of sight
    let distance = current_pos.distance(target_pos);
    let range = range.into();
    if distance <= range {
        let octant = current_pos.octant_to(target_pos);
        let direction = Direction::from_octant(octant);

        Fov::ShadowcastDirection(direction).within_fov(
            current_pos,
            target_pos,
            range,
            map_manager,
            VisionPassThroughData { vision_type },
        )
    } else {
        false
    }
}
