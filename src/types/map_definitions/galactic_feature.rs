use crate::prelude::*;

pub enum GalacticFeatureType {
    Planet,
    Asteroid,
    Starbase,
    MilitaryOutpost,
    TradeOutpost,
    Star,
}

impl GalacticFeatureType {
    
}

pub struct GalacticFeature {
    pub feature_type: GalacticFeatureType,
    pub tile_size: Vec2,
    pub tileset_texture: Handle<Image>,
    pub tile_ids: Vec<Vec<u32>>,
}

impl GalacticFeature {
    pub fn new(feature_type: GalacticFeatureType, tileset: &Tileset, seed: u64) -> Self {
        let rng = Pcg64::from_seed(seed);

        // TODO: implement multiple sized maps based on GalacticFeatureType
        let size = PLANET_SIZE[0].0;

        let tile_ids = Vec::with_capacity(size.x);
        for x in 0..size.x {
            let tiles = Vec::with_capacity(size.y);
            for y in 0..size.y {
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