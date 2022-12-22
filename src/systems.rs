pub mod init {
    mod ui {
        mod egui;
        pub use egui::*;
        mod main_menu;
        pub use main_menu::*;
        mod splash;
        pub use splash::*;
    }
    pub use ui::*;

    mod assets;
    pub use assets::*;
    mod camera;
    pub use camera::*;
    mod cleanup;
    pub use cleanup::*;
    mod input;
    pub use input::*;
    mod generator_config;
    pub use generator_config::*;
    mod spawn_grid;
    pub use spawn_grid::*;
}
pub(super) use init::*;

pub mod functions {
    mod create_tilemap;
    pub use create_tilemap::*;
    mod generate;
    pub use generate::*;
    mod styles;
    pub use styles::*;
}
pub(super) use functions::*;

pub mod quit {
    mod save_app_settings;
    pub use save_app_settings::*;
}
pub(super) use quit::*;

pub mod run {
    mod update_app_settings;
    pub use update_app_settings::*;
    mod update_camera_dimensions;
    pub use update_camera_dimensions::*;
    mod update_tilemap;
    pub use update_tilemap::*;
    mod update_window;
    pub use update_window::*;
}
pub(super) use run::*;

// In prelude
pub(super) mod system_params {
    mod assets;
    pub use assets::*;
    mod app_settings;
    pub use app_settings::*;
}

mod ui {
    mod main_menu;
    pub use main_menu::*;
    mod settings;
    pub use settings::*;
    mod world_gen;
    pub use world_gen::*;
    mod test_menu;
    pub use test_menu::*;
}
pub use ui::*;

mod systems_plugin;
pub(super) use systems_plugin::*;
