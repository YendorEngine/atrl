use crate::prelude::*;
#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Movement(pub u8);

impl From<Vec<MovementType>> for Movement {
    fn from(value: Vec<MovementType>) -> Self { Self(value.iter().fold(0, |a, b| a | b.as_u8())) }
}
