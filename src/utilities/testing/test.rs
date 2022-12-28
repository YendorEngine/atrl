use crate::{add_external_plugins, prelude::*, resources::app_settings::AppSettingsResource, systems::*};

const MAIN_THREAD: &str = "main";
const TEST_FPS: f32 = 144.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestState {
    Init,
    Testing,
}

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
    fn frames() -> u64 { 1 }

    #[allow(unused_variables)]
    /// check the results of the test
    fn check(app: &App, val: A) {}

    /// setup the test
    fn setup(app: &mut App) -> A;

    /// setup the graphics for the test
    fn setup_graphics(app: &mut App) {
        AppState::set_splash_screen_to_state(AppState::Testing(TestState::Testing));

        // Default graphical setup
        app.add_loopless_state(AppState::Testing(TestState::Init))
            .insert_resource(AppSettingsResource::load(false))
            .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
            .add_startup_system_set(
                ConditionSet::new().with_system(init_assets).with_system(init_ui_camera).into(),
            )
            .add_system(wait_for_assets_to_load.run_in_state(AppState::Testing(TestState::Init)));
    }

    /// is the test running on the main thread
    fn on_main_thread() -> bool { matches!(thread::current().name(), Some(MAIN_THREAD)) }

    /// get the app and whether it is running on the main thread
    fn app() -> (App, bool) {
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
    fn run() {
        let (mut app, on_main_thread) = Self::app();
        if on_main_thread {
            Self::setup_graphics(&mut app);
        }

        let res = Self::setup(&mut app);

        if on_main_thread {
            app.run();
        } else {
            for _ in 0..Self::frames() {
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
            Self::check(&app, res)
        }
    }
}
