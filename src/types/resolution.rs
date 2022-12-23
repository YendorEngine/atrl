use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(tag = "resolution")]
pub enum Resolution {
    R720,
    #[default]
    R1080,
    R1440,
    R2160,
    R2160p,
    Custom {
        width: f32,
        height: f32,
    },
}

impl Resolution {
    pub fn new_custom(width: f32, height: f32) -> Self { Self::Custom { width, height } }

    pub fn get_name(&self) -> String {
        match self {
            Resolution::R720 => "720p".to_string(),
            Resolution::R1080 => "1080p".to_string(),
            Resolution::R1440 => "1440p".to_string(),
            Resolution::R2160 => "2160p".to_string(),
            Resolution::R2160p => "2160p+".to_string(),
            Resolution::Custom { width, height } => format!("{width}x{height}"),
        }
    }

    pub fn get_dimensions(&self) -> Vec2 {
        match self {
            Resolution::R720 => Vec2 { x: 1280., y: 720. },
            Resolution::R1080 => Vec2 { x: 1920., y: 1080. },
            Resolution::R1440 => Vec2 { x: 2560., y: 1440. },
            Resolution::R2160 => Vec2 { x: 3840., y: 2160. },
            Resolution::R2160p => Vec2 { x: 4096., y: 2160. },
            Resolution::Custom { width, height } => Vec2 {
                x: *width,
                y: *height,
            },
        }
    }

    pub fn iter() -> impl Iterator<Item = &'static Resolution> {
        [
            Resolution::R720,
            Resolution::R1080,
            Resolution::R1440,
            Resolution::R2160,
            Resolution::R2160p,
        ]
        .iter()
    }
}

impl From<Vec2> for Resolution {
    fn from(resolution: Vec2) -> Self {
        match [resolution.x as u32, resolution.y as u32] {
            [1280, 720] => Resolution::R720,
            [1920, 1080] => Resolution::R1080,
            [2560, 1440] => Resolution::R1440,
            [3840, 2160] => Resolution::R2160,
            [4096, 2160] => Resolution::R2160p,
            [width, height] => Resolution::Custom {
                width: width as f32,
                height: height as f32,
            },
        }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resolution_name = self.get_name();
        write!(f, "{resolution_name}")
    }
}
