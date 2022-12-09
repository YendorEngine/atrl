use crate::prelude::*;
#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Vision(pub u8);

impl From<Vec<u8>> for Vision {
    fn from(value: Vec<u8>) -> Self { Self(value.iter().fold(0, |a, b| a | b)) }
}

impl From<Vec<VisionType>> for Vision {
    fn from(value: Vec<VisionType>) -> Self { Self(value.iter().fold(0, |a, b| a | b.as_u8())) }
}
