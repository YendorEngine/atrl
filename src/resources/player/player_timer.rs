use std::time::Duration;

use crate::{prelude::*, resources::*};

pub struct PlayerTimer {
    pub input_delay_timer: Timer,
    pub unsafe_delay_timer: Option<Timer>,
}

impl FromWorld for PlayerTimer {
    fn from_world(world: &mut World) -> Self {
        world.get_resource::<GameSettings>().map_or_else(
            || {
                error!("PlayerTimer resource needs to be inserted after GameSettings ressource.");
                Self::new()
            },
            |game_settings| Self {
                unsafe_delay_timer: None,
                input_delay_timer: Timer::new(game_settings.pressed_duration(), TimerMode::Once),
            },
        )
    }
}

impl PlayerTimer {
    fn new() -> Self {
        Self {
            unsafe_delay_timer: None,
            input_delay_timer: Timer::new(Duration::from_millis(150), TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct UnsafeInput;
