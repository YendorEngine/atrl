use bevy::diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;
/// This system will then change the title during execution
pub fn set_debug_title(
    mut windows: ResMut<Windows>,
    diagnostics: Res<Diagnostics>,
    state: Res<CurrentGameState>,
) {
    if let Some(window) = windows.get_primary_mut() {
        let title = format!(
            "Avg. FPS: {:.02} | Entity Count: {} | CurrentState: {:?}",
            diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap().average().unwrap_or_default(),
            diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT).unwrap().value().unwrap_or_default(),
            state.0
        );
        window.set_title(title);
    }
}
