#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpiralConfig {
    pub max_stars: u32,
    pub arm_count: u32, // 1 - 6
    pub draw_lines: bool,
}

impl Default for SpiralConfig {
    fn default() -> Self {
        Self {
            arm_count: 6, // 1 - 6
            max_stars: 100,
            draw_lines: false,
        }
    }
}
