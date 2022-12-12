use crate::prelude::{
    systems::{init::*, *},
    *,
};

#[derive(SystemLabel)]
enum MapLabel {
    SwitchMaps,
}

#[derive(Clone, Copy)]
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize states an move to next state immediately
        app.add_loopless_state(AppState::Initializing).add_enter_system(
            AppState::Initializing,
            switch_app_state!(AppState::Loading(LoadingState::Assets)),
        );

        // Register external plugins
        self.external_plugins(app);

        // Register the rest of the systems...
        self.loading_state(app).menu_state(app).game_state(app).quit_state(app);
    }
}

impl SystemsPlugin {
    fn external_plugins(self, app: &mut App) -> Self {
        app
        // ECS Tilemap
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: GRID_SIZE,
        })
        .add_plugin(TilemapPlugin)
        // Big Brain
        .add_plugin(BigBrainPlugin);
        self
    }

    // LoadingState
    fn loading_state(self, app: &mut App) -> Self {
        // These are the first systems to run:
        // Get a camera spawned first, followed by the splash screen so we don't look broken!
        app.add_enter_system_set(
            AppState::Loading(LoadingState::Assets),
            ConditionSet::new()
            .with_system(spawn_cameras) // TODO: Rewrite camera stuff
            //.with_system(show_splash_screen) // TODO: Splash screen
            .with_system(init_white_pixel) // We can use white_pixel as a solid color
            .with_system(init_app_settings) // Load the app_settings from file or default. These will help determine how our app is setup.
            .with_system(switch_app_state!(AppState::Menu(MenuState::MainMenu)))
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
            AppState::Loading(LoadingState::InitGame),
            ConditionSet::new()
            //.with_system(show_loading_screen) // TODO: Loading Screen
            .with_system(init_game_contexts) // Load a saved game or start tracking a new instance
            .with_system(init_mouse_position) // Start tracking the MousePosition
            .with_system(init_map_manager) // Track maps in the game as we move about
            .with_system(init_turn_manager) // Track mobs in game through their turns
            .with_system(switch_app_state!(AppState::Loading(LoadingState::WorldGen)))
            .into(),
        );

        // Generate the overall condition of the world
        app.add_enter_system_set(
            AppState::Loading(LoadingState::WorldGen),
            ConditionSet::new()
                .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(
                    MapGenState::Terrain
                ))))
                .into(),
        );

        // Entry point for generating new maps?
        // Load the terrain for the map
        app.add_enter_system_set(
            AppState::Loading(LoadingState::MapGen(MapGenState::Terrain)),
            ConditionSet::new()
                .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(
                    MapGenState::Features
                ))))
                .into(),
        );

        // Load the features for the map
        app.add_enter_system_set(
            AppState::Loading(LoadingState::MapGen(MapGenState::Features)),
            ConditionSet::new()
                .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(
                    MapGenState::Items
                ))))
                .into(),
        );

        // Load the items for the map
        app.add_enter_system_set(
            AppState::Loading(LoadingState::MapGen(MapGenState::Items)),
            ConditionSet::new()
                .with_system(switch_app_state!(AppState::Loading(LoadingState::MapGen(
                    MapGenState::Actors
                ))))
                .into(),
        );

        // Load the actors for the map
        app.add_enter_system_set(
            AppState::Loading(LoadingState::MapGen(MapGenState::Actors)),
            ConditionSet::new()
                .with_system(spawn_player)
                .with_system(spawn_ai)
                .with_system(switch_app_state!(AppState::Loading(LoadingState::Ready)))
                .into(),
        );

        // Add UI (message?) to Loading Screen letting the player know the game is ready and wait for
        // input
        app.add_enter_system_set(
            AppState::Loading(LoadingState::Ready),
            ConditionSet::new().with_system(switch_app_state!(AppState::InGame)).into(),
        );

        // app.add_exit_system_set(
        // AppState::Loading(LoadingState::Ready),
        // ConditionSet::new()
        // .with_system(despawn_loading_screen) // TODO: Loading Screen
        // .into()
        // );
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
    fn quit_state(self, app: &mut App) -> Self {
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
