use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(tag = "resolution")]
pub enum Resolution {
    #[default]
    R720,
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

/////////////////////
/// Impls
/////////////////////

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resolution_name: String = (*self).into();
        write!(f, "{resolution_name}")
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

impl From<Resolution> for Vec2 {
    fn from(value: Resolution) -> Self {
        match value {
            Resolution::R720 => Vec2::new(1280.0, 720.0),
            Resolution::R1080 => Vec2::new(1920.0, 1080.0),
            Resolution::R1440 => Vec2::new(2560.0, 1440.0),
            Resolution::R2160 => Vec2::new(3840.0, 2160.0),
            Resolution::R2160p => Vec2::new(4096.0, 2160.0),
            Resolution::Custom { width, height } => Vec2::new(width, height),
        }
    }
}

impl From<Resolution> for String {
    fn from(value: Resolution) -> Self {
        match value {
            Resolution::R720 => "720p".to_string(),
            Resolution::R1080 => "1080p".to_string(),
            Resolution::R1440 => "1440p".to_string(),
            Resolution::R2160 => "2160p".to_string(),
            Resolution::R2160p => "2160p+".to_string(),
            Resolution::Custom { width, height } => format!("{width}x{height}"),
        }
    }
}
