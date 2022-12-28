use atrl::{
    components::*,
    prelude::{
        testing::{systems::*, *},
        *,
    },
    systems::{functions::*, *},
    types::asset_ids::tilesets::*,
};

const STAR_ID: u32 = TILE_TG_WORLD_STARS_A_ID;
const EMPTY_ID: u32 = TILE_TG_WORLD_FLOOR_TILE_A_ID;
const CENTER_ID: u32 = TILE_TG_WORLD_ACID_ID;

////////////////////////////////////////////////////////////////////////////////

fn main() { UniverseGenerationTest::run(); }

struct UniverseGenerationTest;

impl Test<()> for UniverseGenerationTest {
    fn frames() -> u64 { 1 }

    fn setup(app: &mut App) -> () {
        app.add_enter_system_set(
            AppState::Testing(TestState::Testing),
            ConditionSet::new().with_system(init_generator_config).with_system(spawn_grid).into(),
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
