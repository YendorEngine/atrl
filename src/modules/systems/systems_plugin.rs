use crate::prelude::{
    *,
    systems::{
        *,
        init,
    },
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize states an move to next state immediately
        app.add_loopless_state(AppState::Initializing).add_enter_system(
            AppState::Initializing,
            switch_app_state!(AppState::Loading(LoadingState::Assets)),
        );

        // LoadingState
        {
            app.add_enter_system_set(
                AppState::Loading(LoadingState::Assets),
                ConditionSet::new()
                .with_system(spawn_cameras) // TODO: Rewrite camera stuff
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::Assets))
                .with_system(init_white_pixel)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::InitGame))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::WorldGen))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Terrain)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Features)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Items)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::MapGen(MapGenState::Actors)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Loading(LoadingState::Ready))
                .with_system(system)
                .into()
            );
        }
        
        // MenuState
        {
            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::MainMenu))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::Settings))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::WorldCreation(WorldCreationState::Screen1)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::WorldCreation(WorldCreationState::Screen2)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::CharacterCreation(CharacterCreationState::RaceClass)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::CharacterCreation(CharacterCreationState::Attributes)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::CharacterCreation(CharacterCreationState::Skills)))
                .with_system(system)
                .into()
            );

            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Menu(MenuState::CharacterCreation(CharacterCreationState::Feats)))
                .with_system(system)
                .into()
            );
        }


        // GameState
        {
            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(system)
                .into()
            );
        }
        
        // Quit
        {
            app.add_system_set_to_stage(
                CoreStage::Update,
                ConditionSet::new()
                .run_in_state(AppState::Quit)
                .with_system(system)
                .into()
            );
        }
    }
}