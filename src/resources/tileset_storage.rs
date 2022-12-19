use crate::prelude::*;

#[derive(Resource)]
pub struct TilesetStorageResource(pub Vec<Handle<Tileset>>);