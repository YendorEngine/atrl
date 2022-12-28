use crate::prelude::*;

pub enum GalacticFeatureType {
    Star,
    Planet,
    Asteroid,
    Starbase,
    TradeOutpost,
    MilitaryOutpost,
}

impl GalacticFeatureType {}

pub struct GalacticFeature {
    pub feature_type: GalacticFeatureType,
    pub tile_size: Vec2,
    pub tileset_texture: Handle<Image>,
    pub tile_ids: Vec<Vec<u32>>,
}

impl GalacticFeature {
    pub fn new(feature_type: GalacticFeatureType, tileset: &Tileset, seed: [u8; 32]) -> Self {
        let _rng = Pcg64::from_seed(seed);

        // TODO: implement multiple sized maps based on GalacticFeatureType
        let size = PLANET_SIZES[0].0;

        let mut tile_ids = Vec::with_capacity(size.x as usize);
        for _x in 0..size.x {
            let mut tiles = Vec::with_capacity(size.y as usize);
            for _y in 0..size.y {
                tiles.push(0);
            }
            tile_ids.push(tiles);
        }

        Self {
            feature_type,
            tile_size: tileset.tile_size(),
            tileset_texture: tileset.texture().clone(),
            tile_ids,
        }
    }
}
