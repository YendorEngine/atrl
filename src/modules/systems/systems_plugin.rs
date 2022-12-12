use crate::prelude::{
    systems::{init::*, *},
    *,
};

#[derive(Clone, Copy)]
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize states an move to next state immediately
        app.add_loopless_state(AppState::Initializing).add_enter_system(
            AppState::Initializing,
            switch_app_state!(AppState::Loading(LoadingState::Assets)),
        );

        // Register the rest of the systems...
        self.loading_state(app).menu_state(app).game_state(app).quit_state(app);
    }
}

impl SystemsPlugin {
    // LoadingState
    fn loading_state(self, app: &mut App) -> Self {
        // These are the first systems to run:
        // Get a camera spawned first, followed by the splash screen so we don't look broken!
        app.add_enter_system_set(
            AppState::Loading(LoadingState::Assets),
            ConditionSet::new()
            .with_system(spawn_cameras) // TODO: Rewrite camera stuff
            //.with_system(show_splash_screen) // TODO: Splash screen
            .into(),
        );

        // Load all of the assets here while the splash screen shows
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::Assets))
                .with_system(init_white_pixel) // We can use white_pixel as a solid color
                .with_system(init_app_settings) // Load the app_settings from file or default. These will help determine how our app is setup.
                .with_system(switch_app_state!(AppState::Menu(MenuState::MainMenu)))
                .into(),
        );

        /*
        // Cleanup Splash Screen
        app.add_exit_system_set(
            AppState::Loading(LoadingState::Assets),
            ConditionSet::new()
            //.with_system(despawn_splash_screen) // TODO: Splash screen
            .into()
        );
        */


        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::InitGame))
            .with_system(init_contexts) // Load a saved game or start tracking a new instance
            .with_system(init_mouse_position) // Start tracking the MousePosition
            .with_system(init_turn_manager) // Track mobs in game through their turns
            .with_system(switch_app_state!(AppState::Loading(LoadingState::WorldGen)))
            .into(),
        );

        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::WorldGen))
            .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(MapGenState::Terrain))))
            .into(),
        );
        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Terrain)))
            .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(MapGenState::Features))))
            .into(),
        );
        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Features)))
            .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(MapGenState::Items))))
            .into(),
        );
        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Items)))
            .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(MapGenState::Actors))))
            .into(),
        );
        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Actors)))
            .with_system(switch_app_state!(AppState::Loading(LoadingState::Ready)))
            .into(),
        );
        
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
            .run_in_state(AppState::Loading(LoadingState::Ready))
            .with_system(switch_app_state!(AppState::InGame))
            .into(),
        );
        self
    }

    // MenuState
    fn menu_state(self, app: &mut App) -> Self {
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new().run_in_state(AppState::Menu(MenuState::MainMenu))
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
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new().run_in_state(AppState::InGame).with_system(system).into(),
        // );
        self
    }

    // Quit
    fn quit_state(self, app: &mut App) -> Self {
        // app.add_system_set_to_stage(
        // CoreStage::Update,
        // ConditionSet::new().run_in_state(AppState::Quit).with_system(system).into(),
        // );
        self
    }
}
