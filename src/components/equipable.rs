use crate::{prelude::*, types::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Equipable {
    pub location: Option<EquipmentSlot>,
    pub available_locations: Vec<EquipmentSlot>,
}
