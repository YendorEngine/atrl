mod action_queue;
pub use action_queue::*;
mod ai_context;
pub use ai_context::*;
mod app_settings;
pub use app_settings::*;
mod camera_settings_resource;
pub use camera_settings_resource::*;
mod loaded_cameras;
pub use loaded_cameras::*;
mod game_context;
pub use game_context::*;
mod map_manager_resource;
pub use map_manager_resource::*;
mod mouse_position;
pub use mouse_position::*;
mod player_entity;
pub use player_entity::*;
mod turn_manager;
pub use turn_manager::*;

use crate::prelude::{
    *,
    resources::{
        ai_context::*,
        app_settings::*,
        game_context::*,
        mouse_position::*,
        turn_manager::*,
    },
};
// TODO: How much of this can we turn into loading systems to hide behind a Splash Screen / Loading Screen?
pub fn setup_resources(app: &mut App) {
    app.init_resource::<AppSettings>();
    app.init_resource::<GameContext>();
    app.init_resource::<AiContext>();
    app.init_resource::<TurnManager>();
    // MousePosition
    // app.init_resource::<>()
}
