use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
};

use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DebugUIState {
    pub show: bool,
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Fps / Entity Tracking
        app.add_plugin(FrameTimeDiagnosticsPlugin).add_plugin(EntityCountDiagnosticsPlugin);

        // Inspector Egui
        app.add_plugin(DefaultInspectorConfigPlugin).add_plugin(EguiPlugin);

        // Systems
        app.init_resource::<DebugUIState>().add_system(inspector_ui).add_system(set_debug_title);
    }
}
