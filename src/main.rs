#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME
#![feature(trait_alias)]
#![feature(adt_const_params)]

#[cfg(feature = "debug")]
mod debug {
    mod systems {
        mod show_ui;
        pub use show_ui::*;
        mod window_title;
        pub use window_title::*;
    }
    pub use systems::*;

    mod debug;
    pub use debug::*;
}

mod log;
use log::*;

mod globals;
// "Modules"
mod modules;

// "Exports" from crate
pub mod imports;
pub mod prelude;
use prelude::{plugins::*, resources::*, *};

fn main() {
    let mut app = App::new();

    // Default Plugins
    default_plugins(&mut app).insert_resource(ClearColor(Color::BLACK));

    // anything we don't need in release versions
    #[cfg(feature = "debug")]
    app.add_plugin(debug::DebugPlugin);

    // game related
    setup_resources(&mut app); // TODO: How much of this can we turn into loading systems to hide behind a Splash Screen /
                               // Loading Screen?
    load_plugins(&mut app); // TODO: How much of this can we turn into loading systems to hide behind a Splash Screen /
                            // Loading Screen?

    app.run();
}

fn default_plugins(app: &mut App) -> &mut App {
    let defaults = DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                title: APP_NAME.to_string(),
                width: (GRID_WIDTH * SPRITE_WIDTH) as f32 * INITIAL_WINDOW_SCALE,
                height: (GRID_HEIGHT * SPRITE_HEIGHT) as f32 * INITIAL_WINDOW_SCALE,
                resize_constraints: WindowResizeConstraints {
                    min_width: (GRID_WIDTH * SPRITE_WIDTH) as f32,
                    min_height: (GRID_HEIGHT * SPRITE_HEIGHT) as f32,
                    ..Default::default()
                },
                // present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(atrl_log_plugin())
        .build();

    app.add_plugins(defaults);

    #[cfg(feature = "release")]
    {
        defaults.add_before::<bevy::asset::AssetPlugin, _>(bevy_embedded_assets::EmbeddedAssetPlugin);
    }

    app
}
