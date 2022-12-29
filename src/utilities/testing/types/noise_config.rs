#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoiseConfig {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub cutoff: f64,

    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            cutoff: 0.5,

            octaves: 6,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }
}
