use crate::{
    init_resource, prelude::*, resources::universe_generation_settings::UniverseGenerationSettings,
    systems::*,
};

// TODO: Remove this when we want to finalize going to MainMenu.
pub const SPLASH_SCREEN_TO_THIS_STATE: AppState = AppState::Menu(Main);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Initializing,
    SplashScreen,
    Menu(MenuState),
    InGame,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuState {
    Main,
    Settings,
    UniverseGeneration,
}
pub use MenuState::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        self.init_states(app);
        self.continuous_state(app);
        self.loading_state(app);
        self.menu_state(app);
        self.game_state(app);
        self.quit_state(app);
    }
}

// TODO: remove - allows .with_system() to hold system creation
// fn phantom_system() {}

impl SystemsPlugin {
    fn init_states(&self, app: &mut App) {
        app.add_loopless_state(AppState::Initializing).add_enter_system(
            AppState::Initializing,
            switch_app_state!(AppState::SplashScreen),
        );
    }

    fn continuous_state(&self, app: &mut App) {
        // Update App Settings
        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_not_in_state(AppState::SplashScreen)
                .with_system(update_app_settings)
                .with_system(update_window)
                .into(),
        );
    }

    fn loading_state(&self, app: &mut App) {
        // Camera / SplashScreen? / Assets
        app.add_enter_system_set(
            AppState::SplashScreen,
            ConditionSet::new()
                .with_system(init_assets)
                .with_system(init_ui_camera)
                .with_system(init_splash_screen)
                .into(),
        );

        // Wait for Assets
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(AppState::SplashScreen)
                .with_system(wait_for_assets_to_load)
                .into(),
        );

        // Resources
        app.add_exit_system_set(
            AppState::SplashScreen,
            ConditionSet::new().with_system(init_egui).with_system(cleanup_on_exit_splash).into(),
        );
    }

    fn menu_state(&self, app: &mut App) {
        // Main Menu
        app.add_enter_system_set(
            AppState::Menu(MenuState::Main),
            ConditionSet::new().with_system(init_main_menu).into(),
        )
        .add_system_set(
            ConditionSet::new().run_in_state(AppState::Menu(MenuState::Main)).with_system(main_menu).into(),
        );

        // Settings Menu
        app.add_system_set(
            ConditionSet::new().run_in_state(AppState::Menu(Settings)).with_system(settings_menu).into(),
        );

        // Universe Gen Menu
        app.add_enter_system_set(
            AppState::Menu(UniverseGeneration),
            ConditionSet::new().with_system(init_resource!(UniverseGenerationSettings)).into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Menu(UniverseGeneration))
                .with_system(universe_gen_menu)
                .into(),
        );
    }

    fn game_state(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::InGame,
            ConditionSet::new()
                .with_system(init_input)
                .with_system(spawn_grids)
                .with_system(init_generator_config)
                .with_system(cleanup_on_exit_main_menu)
                .into(),
        );

        app.add_system_set_to_stage(
            CoreStage::First,
            ConditionSet::new().run_in_state(AppState::InGame).with_system(update_tilemap).into(),
        );
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new().run_in_state(AppState::InGame).with_system(test_menu).into(),
        );
        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new().run_in_state(AppState::InGame).with_system(update_camera_dimensions).into(),
        );
    }

    fn quit_state(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new().run_on_event::<AppExit>().with_system(save_app_settings).into(),
        );
    }
}
