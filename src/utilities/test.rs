use crate::{add_external_plugins, prelude::*, resources::app_settings::AppSettingsResource, systems::*};

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
pub trait Test<A> {
    /// number of frames to run the test for (if not on main thread)
    fn frames(&self) -> u64 { 1 }

    #[allow(unused_variables)]
    /// check the results of the test
    fn check(&self, app: &App, val: A) {}

    /// setup the test
    fn setup(&self, app: &mut App) -> A;

    /// setup the graphics for the test
    fn setup_graphics(&self, app: &mut App) {
        // Default graphical setup
        app.insert_resource(AppSettingsResource::load(false));
        app.add_startup_system(init_ui_camera);
        app.world.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));
    }

    /// is the test running on the main thread
    fn on_main_thread(&self) -> bool { matches!(thread::current().name(), Some(MAIN_THREAD)) }

    /// get the app and whether it is running on the main thread
    fn app(&self) -> (App, bool) {
        let mut app = App::new();
        let on_main_thread = if self.on_main_thread() {
            println!("Test running on main thread, will display window");
            true
        } else {
            println!("Test not running on main thread, will run headlessly");
            false
        };

        if on_main_thread {
            app.add_plugins(DefaultPlugins);
            add_external_plugins(&mut app);
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

    /// run the test
    fn run(&self)
    where Self: Sized {
        let (mut app, on_main_thread) = self.app();
        if on_main_thread {
            self.setup_graphics(&mut app);
        }

        let res = self.setup(&mut app);

        if on_main_thread {
            app.run();
        } else {
            for _ in 0..self.frames() {
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
            self.check(&app, res)
        }
    }
}
