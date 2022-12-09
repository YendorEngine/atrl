use bevy::reflect::{FromReflect, Reflect};

#[derive(Reflect, FromReflect, Default, Debug)]
pub enum EquipmentSlot {
    #[default]
    Head,
    Chest,
    Shoulders,
    Back,
    Arms,
    Hands,
    Waist,
    Legs,
    Feet,
}
