#[repr(u8)]
pub enum MapLayer {
    Unknown = 0,
    Terrain = 1,
    Features,
    Items,
    Actors,
    Player,
    UI,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> Self { (value as u16 * 2 - 1) as Self } // spread the map layers out incase we need extra layers.
}
