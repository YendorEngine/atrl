use crate::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct CanSeePlayer {
    pub score_if_true: f32,
}

impl Default for CanSeePlayer {
    fn default() -> Self { Self { score_if_true: 1.0 } }
}
