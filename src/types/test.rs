use crate::{prelude::*, systems::init::init_ui_camera};

const MAIN_THREAD: &str = "main";

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

impl<A> Test<A> {
    pub fn on_main_thread() -> bool { matches!(thread::current().name(), Some(MAIN_THREAD)) }

    pub fn default_setup_graphics(app: &mut App) {
        app.add_startup_system(init_ui_camera);

        app.world.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));
    }

    pub fn app(&self) -> (App, bool) {
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
            app.add_plugins(TestPlugins);
        }

        // For Debugging
        #[cfg(feature = "debug")]
        app.add_plugin(WorldInspectorPlugin::new());

        (app, on_main_thread)
    }

    pub fn run(self) {
        let (mut app, on_main_thread) = self.app();

        if on_main_thread {
            (self.setup_graphics)(&mut app);
        }

        let res = (self.setup)(&mut app);

        if on_main_thread {
            app.run();
        } else {
            for _ in 0..self.frames {
                app.update();
            }
            (self.check)(&app, res)
        }
    }
}
