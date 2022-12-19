use crate::prelude::*;

#[derive(Component, Reflect, Default, Serialize, Deserialize, Clone, Copy, Deref, DerefMut)]
pub struct LocalPositionComponent(pub UVec2);
