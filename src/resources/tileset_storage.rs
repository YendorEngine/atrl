use crate::prelude::*;

#[derive(Resource)]
pub struct TilesetStorage(pub Vec<Handle<Tileset>>);
