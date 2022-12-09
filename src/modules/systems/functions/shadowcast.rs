// FoV implementation taken from:
// https://www.albertford.com/shadowcasting/
use crate::prelude::*;

pub struct Shadowcast;

impl<'w, 's> FovAlgorithm<'w, 's> for Shadowcast {
    fn compute_fov(
        origin: Position,
        vision_type: u8,
        range: u32,
        provider: &mut impl FovProvider,
        q_blocks_vision: &Query<'w, 's, &'static BlocksVision>,
        receiver: &mut impl FovReceiver,
    ) {
        receiver.set_visible(origin);
        CardinalDirection::all().enumerate().for_each(|(_index, direction)| {
            let mut quadrant = Quadrant::new(
                direction,
                origin,
                vision_type,
                provider,
                q_blocks_vision,
                receiver,
            );
            let mut first_row = ShadowcastRow::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row);
        });
    }
}
impl<'w, 's> Shadowcast {
    pub fn compute_direction(
        origin: Position,
        vision_type: u8,
        range: u32,
        provider: &mut impl FovProvider,
        q_blocks_vision: &Query<'w, 's, &'static BlocksVision>,
        receiver: &mut impl FovReceiver,
        direction: CardinalDirection,
    ) {
        receiver.set_visible(origin);
        let mut quadrant = Quadrant::new(
            direction,
            origin,
            vision_type,
            provider,
            q_blocks_vision,
            receiver,
        );
        let mut first_row = ShadowcastRow::new(1, Slope::new(-1, 1), Slope::new(1, 1));
        Self::scan_recursive(range, &mut quadrant, &mut first_row);
    }

    fn scan_recursive(range: u32, quadrant: &mut Quadrant, row: &mut ShadowcastRow) {
        let mut prev_tile = None;
        for tile in row.tiles() {
            if quadrant.distance_squared(tile) > (range as u64 * range as u64) {
                continue;
            }

            // Should we reveal the tile?
            if quadrant.is_opaque(tile) | row.is_symmetric(tile) {
                quadrant.set_visible(tile);
            }

            // handle the current row based on vision angles around the previous tile
            if let Some(prev_tile) = prev_tile {
                // did we *just* hit floor after traveling through walls?
                if quadrant.is_opaque(prev_tile) & quadrant.is_clear(tile) {
                    row.calc_starting_slope(tile)
                }
                // did we *just* hit a wall after traveling through floors?
                if quadrant.is_clear(prev_tile) & quadrant.is_opaque(tile) {
                    let mut next_row = row.next();
                    next_row.calc_ending_slope(tile);
                    Self::scan_recursive(range, quadrant, &mut next_row);
                }
            }

            // setup for next tile
            prev_tile = Some(tile);
        }

        // if our last tile was floor, we can see down another row
        if let Some(prev_tile) = prev_tile {
            if quadrant.is_clear(prev_tile) {
                Self::scan_recursive(range, quadrant, &mut row.next());
            }
        }
    }
}
