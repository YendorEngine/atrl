// Features
#![feature(adt_const_params)]
#![feature(control_flow_enum)]
#![feature(trait_alias)]
// Clippy
#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::module_inception)]

//##############################
// Modules
//##############################

// To use:
// `use crate::XXX:*;`
pub mod components;
pub mod resources;
pub mod systems;
pub mod types;
pub mod ui;
pub mod utilities;

// Included in prelude:
mod globals;
mod imports;

pub mod prelude {
    pub use crate::{globals::*, imports::*, systems::system_params::*, types::definitions::*, utilities::*};
    // Macros
    pub use crate::{
        impl_default, impl_new, insert_resource, remove_resource, spawn_component, switch_app_state,
    };
}

// Just in main:
mod log;

//##############################
// usings:
//##############################
use log::*;
use prelude::*;
use resources::AppSettingsResource;
use systems::systems_plugin::*;

struct AtrlGame {
    app: App,
    app_settings: AppSettingsResource,
}

impl AtrlGame {
    fn new() -> Self {
        let mut app = App::new();
        let app_settings = AppSettingsResource::load();
        Self { app, app_settings }
    }

    fn build(&mut self) -> App {
        let app_settings = AppSettingsResource::load();

        self.add_default_plugins().add_external_plugins();
        self.app.add_plugin(SystemsPlugin).insert_resource(app_settings);

        self.app
    }

    fn add_default_plugins(&mut self) -> &mut Self {
        let UVec2 {
            x: grid_width,
            y: grid_height,
        } = self.app_settings.get_grid_size();
        let UVec2 {
            x: cell_width,
            y: cell_height,
        } = self.app_settings.get_cell_size();
        let UVec2 {
            x: window_width,
            y: window_height,
        } = self.app_settings.get_window_size();

        let window_mode = if self.app_settings.get_fullscreen() {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        };

        self.app
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        window: WindowDescriptor {
                            title: APP_NAME.to_string(),
                            width: window_width as f32,
                            height: window_height as f32,
                            resize_constraints: WindowResizeConstraints {
                                min_width: (grid_width * cell_width) as f32,
                                min_height: (grid_height * cell_height) as f32,
                                ..Default::default()
                            },
                            mode: window_mode,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(atrl_log_plugin()),
            )
            .insert_resource(ClearColor(Color::BLACK));

        self
    }

    fn add_external_plugins(&mut self) -> &mut Self {
        self.app
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: CHUNK_SIZE,
            })
            // Tilemap
            .add_plugin(TilemapPlugin)
            // Kayak
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            // Big Brain
            .add_plugin(BigBrainPlugin);

        self
    }
}

fn main() { AtrlGame::new().build().run(); }
