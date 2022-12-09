use crate::prelude::*;

#[derive(
    Reflect,
    FromReflect,
    Debug,
    Default,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]

#[repr(u8)] // this must match with vision component
#[rustfmt::skip]
pub enum VisionType {
    #[default]
    None        = 0,
    Blind       = 1 << 0,
    Normal     = 1 << 1,
    Infared     = 1 << 2,
    XRay        = 1 << 3,
    NotBlind    = !(VisionType::Blind as u8),
    Any         = !0,
}

impl VisionType {
    pub fn as_u8(self) -> u8 { self.try_into().unwrap_or(Self::None as u8) }

    pub fn from_vec(value: Vec<Self>) -> u8 { value.iter().fold(Self::None.as_u8(), |a, b| a | b.as_u8()) }
}

impl TryFrom<VisionType> for u8 {
    type Error = String;

    fn try_from(value: VisionType) -> Result<Self, Self::Error> {
        value.to_u8().map_or_else(
            || Err("Failed to convert `VisionType` to `u8`".to_string()),
            Ok,
        )
    }
}

impl From<u8> for VisionType {
    fn from(value: u8) -> Self { Self::try_from(value).expect("Failed to convert `u8` to `MovementType`") }
}

impl_as_primative!(VisionType);
