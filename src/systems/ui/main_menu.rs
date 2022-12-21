use crate::{prelude::*, systems::*};

pub fn main_menu(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Away Team Roguelike")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui.button("Start").kbgp_navigation().kbgp_initial_focus().clicked() {
                    switch_app_state!(commands, AppState::InGame)
                }

                if ui.button("Settings").kbgp_navigation().clicked() {
                    switch_app_state!(commands, AppState::Menu(Settings))
                }

                // Quit game option
                if ui.button("Quit").kbgp_navigation().clicked() {
                    exit.send(AppExit);
                }
            });
        });
}
