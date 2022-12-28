use atrl::prelude::*;

fn main() { UniverseGenerationTest {}.run(); }

struct UniverseGenerationTest;

impl Test<()> for UniverseGenerationTest {
    fn frames(&self) -> u64 { 1 }

    fn setup(&self, app: &mut App) -> () {
        app.add_system_set(ConditionSet::new().with_system(|| {}).into());
    }
}
