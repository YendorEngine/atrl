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
    pub use crate::{globals::*, imports::*, systems::system_params::*, types::definitions::*};
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

fn main() {
    let app_settings = AppSettingsResource::load();
    println!("GRID_SIZE: {GRID_SIZE:#?}");
    println!("AppSettings: {:#?}", app_settings.get_grid_size());
    println!("GRID_SIZE2: {GRID_SIZE:#?}");

    let mut app = App::new();

    // Init Bevy
    add_default_plugins(&mut app, &app_settings);
    // Init other plugins
    add_external_plugins(&mut app, &app_settings);

    // Init us
    app.add_plugin(SystemsPlugin);

    // We are done loading app_settings, add it as a resource so we can update it.
    app.insert_resource(app_settings);
    app.run();
}

fn add_default_plugins<'a>(app: &'a mut App, app_settings: &AppSettingsResource) -> &'a mut App {
    let UVec2 {
        x: grid_width,
        y: grid_height,
    } = app_settings.get_grid_size();
    let UVec2 {
        x: sprite_width,
        y: sprite_height,
    } = app_settings.get_sprite_size();
    let UVec2 {
        x: window_width,
        y: window_height,
    } = app_settings.get_window_size();

    let window_mode =
        if app_settings.get_fullscreen() { WindowMode::BorderlessFullscreen } else { WindowMode::Windowed };

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: format!("{APP_NAME} - {APP_NAME_SHORT}"),
                    width: window_width as f32,
                    height: window_height as f32,
                    resize_constraints: WindowResizeConstraints {
                        min_width: (grid_width * sprite_width) as f32,
                        min_height: (grid_height * sprite_height) as f32,
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

    app
}

fn add_external_plugins<'a>(app: &'a mut App, app_settings: &AppSettingsResource) -> &'a mut App {
    app.insert_resource(TilemapRenderSettings {
        render_chunk_size: app_settings.get_grid_size(),
    })
    .add_plugin(TilemapPlugin)
    .add_plugin(KayakContextPlugin)
    .add_plugin(KayakWidgets);

    app
}
