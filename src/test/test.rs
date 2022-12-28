use crate::{prelude::*, systems::init::init_ui_camera};

const MAIN_THREAD: &str = "main";
const TEST_FPS: f32 = 144.0;

struct TestPlugins;

impl PluginGroup for TestPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(bevy::core::CorePlugin::default())
            .add(bevy::app::ScheduleRunnerPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::transform::TransformPlugin)
            .add(bevy::hierarchy::HierarchyPlugin)
            .add(bevy::diagnostic::DiagnosticsPlugin)
            .add(bevy::input::InputPlugin)
            .add(bevy::asset::AssetPlugin::default())
            .add(bevy::scene::ScenePlugin::default())
            .add(bevy::gilrs::GilrsPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::render::texture::ImagePlugin::default())
    }
}

pub struct Test<A> {
    pub frames: u64,              // number of frames to run the test for (if not on main thread)
    pub check: fn(&App, A),       // check the results of the test
    pub setup: fn(&mut App) -> A, // setup the test
    pub setup_graphics: fn(&mut App), // setup the graphics for the test
}

impl Default for Test<()> {
    fn default() -> Self {
        Self {
            frames: 1,
            check: |_, _| {},
            setup: |_| {},
            setup_graphics: Self::default_setup_graphics,
        }
    }
}

impl<A> Test<A> {
    pub fn on_main_thread() -> bool { matches!(thread::current().name(), Some(MAIN_THREAD)) }

    pub fn default_setup_graphics(app: &mut App) {
        app.add_startup_system(init_ui_camera);
        app.world.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));
    }

    pub fn app() -> (App, bool) {
        let mut app = App::new();
        let on_main_thread = if Self::on_main_thread() {
            println!("Test running on main thread, will display window");
            true
        } else {
            println!("Test not running on main thread, will run headlessly");
            false
        };

        if on_main_thread {
            app.add_plugins(DefaultPlugins);
        } else {
            let time = Time::default();
            app.insert_resource(time)
                .insert_resource(bevy::render::settings::WgpuSettings {
                    backends: None,
                    ..default()
                })
                .add_plugins(TestPlugins);
        }

        #[cfg(feature = "debug")]
        app.add_plugin(WorldInspectorPlugin::new());

        (app, on_main_thread)
    }

    pub fn run(self) {
        let (mut app, on_main_thread) = Self::app();
        if on_main_thread {
            (self.setup_graphics)(&mut app);
        }

        let res = (self.setup)(&mut app);

        if on_main_thread {
            app.run();
        } else {
            for _ in 0..self.frames {
                // Update time manually for consistent time.delta()
                let mut time = app.world.resource_mut::<Time>();
                if let Some(last_update) = time.last_update() {
                    time.update_with_instant(last_update + Duration::from_secs_f32(1.0 / TEST_FPS));
                } else {
                    time.update();
                }
                // Run systems
                app.update();
            }
            (self.check)(&app, res)
        }
    }
}
