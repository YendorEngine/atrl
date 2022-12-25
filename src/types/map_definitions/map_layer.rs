pub enum MapLayer {
    Terrain,
    Features,
    Items,
    Actors,
    Ui,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> Self { value as u32 as f32 }
}
