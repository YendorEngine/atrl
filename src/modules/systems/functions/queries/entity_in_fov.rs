use crate::prelude::*;

pub fn entity_in_fov<Range: Into<u32>>(
    map_manager: &mut MapManager,
    q_blocks_vision: &Query<&'static BlocksVision>,
    range: Range,
    vision: &Vision,
    current_pos: Position,
    target_pos: Position,
) -> bool {
    // // If the player is within the FOV range of the AI, check line of sight
    let distance = current_pos.distance(target_pos);
    let range = range.into();
    if distance <= range {
        let octant = current_pos.octant_to(target_pos);
        let direction = Direction::from_octant(octant);
        Fov::ShadowcastDirection(direction).compute(current_pos, range, map_manager, VisionPassThroughData {
            q_blocks_vision,
            vision_type: vision.0,
        });

        false
    } else {
        false
    }
}
