use crate::systems::{init::*, *};

#[derive(SystemLabel)]
enum MapLabel {
    SwitchMaps,
}

#[derive(Clone, Copy)]
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Register the rest of the systems...
        self.init_states(app).loading_state(app).menu_state(app).game_state(app).quit_state(app);
    }
}

impl SystemsPlugin {
    fn init_states(self, app: &mut App) -> Self {
        app.add_loopless_state(AppState::Initializing)
            .add_enter_system(AppState::Initializing, switch_app_state!(AppState::Loading));

        self
    }

    // LoadingState
    fn loading_state(self, app: &mut App) -> Self {
        // Camera / SplashScreen? / Assets
        app.add_enter_system_set(
            AppState::Loading,
            ConditionSet::new()
                .with_system(init_camera)
                .with_system(init_assets)
                //.with_system(show_splash_screen) // TODO: Splash screen
                .with_system(init_white_pixel) // We can use white_pixel as a solid color
            .into(),
        );

        // Cleanup Splash Screen
        // app.add_exit_system_set(
        // AppState::Loading(LoadingState::Assets),
        // ConditionSet::new()
        // .with_system(despawn_splash_screen) // TODO: Splash screen
        // .into()
        // );

        // Show Loading Screen
        app.add_enter_system_set(
            AppState::Loading,
            ConditionSet::new()
            //.with_system(show_loading_screen) // TODO: Loading Screen
            // .with_system(init_game_contexts) // Load a saved game or start tracking a new instance
            .with_system(init_mouse_position) // Start tracking the MousePosition
            .with_system(init_map_manager) // Track maps in the game as we move about
            .with_system(init_turn_manager) // Track mobs in game through their turns
            .with_system(switch_app_state!(AppState::Loading(LoadingState::WorldGen)))
            .into(),
        );

        // Resources
        app.add_exit_system_set(
            AppState::Loading,
            ConditionSet::new()
                .with_system(init_resources)
                //.with_system(init_tilemap)
                .into(),
        );

        self
    }

    // MenuState
    fn menu_state(self, app: &mut App) -> Self {
        app.add_enter_system_set(
            AppState::Menu(MenuState::MainMenu),
            ConditionSet::new()
            // TODO: LoadSavedGame or MenuState WorldCreation ->
            .with_system(switch_app_state!(AppState::Loading(LoadingState::InitGame)))
            .into(),
        );

        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new().run_in_state(AppState::Menu(MenuState::Settings)).with_system(system).
        // into(), );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::WorldCreation(
        // WorldCreationState::Screen1,
        // )))
        // .with_system(system)
        // .into(),
        // );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::WorldCreation(
        // WorldCreationState::Screen2,
        // )))
        // .with_system(system)
        // .into(),
        // );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::CharacterCreation(
        // CharacterCreationState::RaceClass,
        // )))
        // .with_system(system)
        // .into(),
        // );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::CharacterCreation(
        // CharacterCreationState::Attributes,
        // )))
        // .with_system(system)
        // .into(),
        // );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::CharacterCreation(
        // CharacterCreationState::Skills,
        // )))
        // .with_system(system)
        // .into(),
        // );
        //
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new()
        // .run_in_state(AppState::Menu(MenuState::CharacterCreation(
        // CharacterCreationState::Feats,
        // )))
        // .with_system(system)
        // .into(),
        // );
        self
    }

    // GameState
    fn game_state(self, app: &mut App) -> Self {
        self.ai_states(app);

        // Switch then update tilemaps
        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .label(MapLabel::SwitchMaps)
                .with_system(set_current_map_to_current_player)
                .into(),
        );
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .after(MapLabel::SwitchMaps)
                .with_system(update_tilemaps)
                .into(),
        );
        self
    }

    // Quit
    fn quit_state(self, _app: &mut App) -> Self {
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new().run_in_state(AppState::Quit).with_system(system).into(),
        // );
        self
    }

    fn ai_states(self, app: &mut App) -> Self {
        app
            // Scoring Systems
            .add_system_set_to_stage(
                BigBrainStage::Scorers,
                ConditionSet::new()
                .run_in_state(AppState::InGame)
                    .with_system(can_see_player)
                    .into(),
            )
            // Action Systems
            .add_system_set_to_stage(
                AppStage::AIThinking,
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(wander_action)
                    .with_system(chase_action)
                    .with_system(attack_action)
                    .into(),
            );
        self
    }
}
