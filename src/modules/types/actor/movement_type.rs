use crate::prelude::*;

#[derive(
    Reflect,
    FromReflect,
    Default,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Debug,
    FromPrimitive,
    ToPrimitive,
)]
pub enum MovementType {
    #[default]
    None = 0,
    Walk = 1 << 0,
    Run = 1 << 1,
    Swim = 1 << 2,
    Fly = 1 << 3,
    Phase = 1 << 4,
    Any = !0,
}

impl MovementType {
    pub fn as_u8(self) -> u8 { self.try_into().unwrap_or(Self::None as u8) }

    pub fn from_vec(value: Vec<Self>) -> u8 { value.iter().fold(Self::None.as_u8(), |a, b| a | b.as_u8()) }
}

impl TryFrom<MovementType> for u8 {
    type Error = String;

    fn try_from(value: MovementType) -> Result<Self, Self::Error> {
        value.to_u8().map_or_else(
            || Err("Failed to convert `MovementType` to `u8`".to_string()),
            Ok,
        )
    }
}

impl From<u8> for MovementType {
    fn from(value: u8) -> Self { Self::try_from(value).expect("Failed to convert `u8` to `MovementType`") }
}

impl_as_primative!(MovementType);
