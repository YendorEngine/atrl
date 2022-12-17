#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MapLayer {
    Terrain = 0,
    Features = 1,
    Items = 2,
    Actors = 3,
    Ui = 4,
}

impl From<MapLayer> for f32 {
    fn from(value: MapLayer) -> Self { value as u32 as f32 }
}
