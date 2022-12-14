pub mod system_params {
    pub mod camera {
        mod cameras;
        pub use cameras::*;
    }

    pub mod map_manager {
        mod map_manager;
        pub use map_manager::*;
    }
    pub use map_manager::*;

    mod blocking_params;
    pub use blocking_params::*;
}

//////////////////////////////////////////////////////////////
pub mod ai {
    mod scorers {
        mod can_see_player;
        pub use can_see_player::*;
    }
    pub use scorers::*;

    mod actions {
        mod attack;
        pub use attack::*;
        mod chase;
        pub use chase::*;
        mod wander;
        pub use wander::*;
    }
    pub use actions::*;
}
pub use ai::*;

pub mod functions {
    mod actions {
        mod try_attack;
        pub use try_attack::*;
        mod try_move;
        pub use try_move::*;
    }
    pub use actions::*;

    pub mod queries {
        mod entity_in_fov;
        pub use entity_in_fov::*;
        mod in_attack_range;
        pub use in_attack_range::*;
    }
    pub use queries::*;

    mod create_tilemap;
    pub use create_tilemap::*;
}

pub mod init {
    mod resources;
    pub use resources::*;
    mod ui;
    pub use ui::*;

    mod map_manager;
    pub use map_manager::*;
    mod mouse_position;
    pub use mouse_position::*;
    mod spawn_cameras;
    pub use spawn_cameras::*;
    mod spawn_mob;
    pub use spawn_mob::*;
    mod turn_manager;
    pub use turn_manager::*;
    mod white_pixel;
    pub use white_pixel::*;
}
// not pub used as these are only needed by `SystemsPlugin`

pub mod run {
    mod player_input;
    pub use player_input::*;
    mod set_current_map_to_current_player;
    pub use set_current_map_to_current_player::*;
    mod store_window_size;
    pub use store_window_size::*;
    mod update_tilesmaps;
    pub use update_tilesmaps::*;
}
pub use run::*;

pub mod quit {
    mod save_settings;
    pub use save_settings::*;
}
pub use quit::*;

pub mod systems_plugin;
// not pub used as this is only needed by "main()"

mod app_state;
pub use app_state::*;
mod app_stage;
pub use app_stage::*;
