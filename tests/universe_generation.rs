use atrl::{
    prelude::{
        testing::{systems::*, *},
        *,
    },
    systems::{init::spawn_grids, *},
    types::asset_ids::tilesets::*,
};

////////////////////////////////////////////////////////////////////////////////

fn main() { UniverseGenerationTest::run(); }

struct UniverseGenerationTest;

impl Test<()> for UniverseGenerationTest {
    fn setup(app: &mut App) -> () {
        app.add_enter_system_set(
            AppState::Testing(TestState::Testing),
            ConditionSet::new().with_system(init_generator_config).with_system(spawn_grids).into(),
        )
        .add_system_set_to_stage(
            CoreStage::First,
            ConditionSet::new()
                .run_in_state(AppState::Testing(TestState::Testing))
                .with_system(update_tilemap)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Testing(TestState::Testing))
                .with_system(universe_generation_menu_test)
                .into(),
        );
    }
}
