use crate::prelude::*;

// Atrl Stage Design
// └───
//     ├── (1) CoreStage::First
//     │   └── BigBrainStage::Scorers
//     │       └── BigBrainStage::Thinkers
//     ├── (2) CoreStage::PreUpdate
//     │   └── AppStage::AIThinking
//     │       └── AppStage::ProcessTurns (perform_turns_sys)
//     │           ├── AppStage::ProcessEvents (movement_sys)
//     │           └── AppStage::ProcessEffects
//     ├── (3) CoreStage::Update
//     ├── (4) CoreStage::PostUpdate
//     ├── (5) AppStage::CleanupEvents
//     └── (6) CoreStage::Last => (cleanup systems: cull_dead, fov, etc)
//         └── BigBrainStage::Cleanup
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AppStage {
    AIThinking,
    ProcessTurns,
    ProccessEvents,
    ProcessEffects,
    CleanupEvents,
}
