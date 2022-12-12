use crate::prelude::*;

pub fn entity_in_fov<'w, 's, Range: Into<u32>>(
    map_manager: &mut MapManager<'w, 's>,
    q_blocks_vision: &Query<'w, 's, &'static BlocksVision>,
    range: Range,
    vision_type: u8,
    current_pos: Position,
    target_pos: Position,
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
            VisionPassThroughData {
                vision_type,
                q_blocks_vision,
            },
        )
    } else {
        false
    }
}
