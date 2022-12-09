use crate::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub max_hp: i32,
    pub current_hp: i32,
}

impl Health {
    pub const fn new(current_hp: i32, max_hp: i32) -> Self { Self { current_hp, max_hp } }

    pub const fn full(max_hp: i32) -> Self {
        Self {
            max_hp,
            current_hp: max_hp,
        }
    }
}
