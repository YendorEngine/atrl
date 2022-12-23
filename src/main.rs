// Features
#![feature(const_for)]
// Clippy
#![allow(clippy::module_inception)]

pub mod components;
pub mod resources;
pub mod systems;
pub mod types;

mod globals;
mod imports;
mod macros;
mod utilities;

pub mod prelude {
    pub use super::{globals::*, imports::*, systems::system_params::*, utilities::*};
    // Macros:
    pub use crate::switch_app_state;
}

use components::register_components;
use resources::app_settings::AppSettingsResource;

use self::{systems::*, types::input::*};
use crate::prelude::*;
fn main() {
    let app_settings = AppSettingsResource::load();
    let mut app = App::new();

    add_default_plugins(&mut app, &app_settings);
    add_external_plugins(&mut app);

    register_components(&mut app);

    app.add_plugin(SystemsPlugin);

    app.insert_resource(app_settings);
    app.run();
}

fn add_default_plugins(app: &mut App, app_settings: &AppSettingsResource) {
    let window_mode = app_settings.window_mode;
    let Vec2 {
        x: window_width,
        y: window_height,
    } = app_settings.window_resolution.get_dimensions();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: APP_NAME.to_string(),
                    width: window_width,
                    height: window_height,
                    resize_constraints: WindowResizeConstraints {
                        min_width: MIN_SCREEN_SIZE.x,
                        min_height: MIN_SCREEN_SIZE.y,
                        ..Default::default()
                    },
                    mode: window_mode,
                    ..Default::default()
                },
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::rgb(0.21, 0.21, 0.21))); // matching tiny_galaxy
                                                                // background
}

fn add_external_plugins(app: &mut App) {
    app.insert_resource(TilemapRenderSettings {
        render_chunk_size: RENDER_CHUNK_SIZE,
    })
    .add_plugin(TilemapPlugin)
    .add_plugin(TilesetPlugin::default())
    .add_plugin(InputManagerPlugin::<MovementInput>::default())
    .add_plugin(EguiPlugin)
    .add_plugin(KbgpPlugin);
}
