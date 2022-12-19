pub mod functions {}
pub use functions::*;

pub mod init {
    mod assets;
    pub use assets::*;
    mod camera;
    pub use camera::*;
    mod cleanup;
    pub use cleanup::*;
    mod input;
    pub use input::*;
}
pub(super) use init::*;

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
}
pub(super) use run::*;

// In prelude
pub(super) mod system_params {
    mod app_settings;
    pub use app_settings::*;
}

mod systems_plugin;
pub(super) use systems_plugin::*;
