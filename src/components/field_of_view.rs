use crate::prelude::*;

#[derive(Reflect, Component, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct FieldOfView(pub u8);
