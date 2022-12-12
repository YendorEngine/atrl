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

pub mod camera {}

pub mod init {
    mod app_settings;
    pub use app_settings::*;
    mod game_contexts;
    pub use game_contexts::*;
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
pub use functions::*;

pub mod run {
    mod player_input;
    pub use player_input::*;
    mod set_current_map_to_current_player;
    pub use set_current_map_to_current_player::*;
    mod update_tilesmaps;
    pub use update_tilesmaps::*;
}
pub use run::*;

pub mod systems_plugin;
