use crate::prelude::*;

#[derive(Debug, Deserialize, Reflect, Default, Clone, Copy)]
pub enum AIType {
    #[default]
    Player,
    Aggressive,
    Scared,
}
