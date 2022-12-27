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
            // ui.visuals_mut().widgets.active.bg_fill = Color32::from_gray(60);
            // ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_gray(60);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .button("Quick Gen")
                    .kbgp_navigation()
                    .kbgp_initial_focus()
                    .kbgp_focus_label(FocusLabel::Initial)
                    .clicked()
                {
                    ui.kbgp_clear_input();
                    switch_app_state!(commands, AppState::InGame)
                }

                if ui.button("World Gen").kbgp_navigation().clicked() {
                    ui.kbgp_clear_input();
                    ui.kbgp_set_focus_label(FocusLabel::Initial);
                    switch_app_state!(commands, AppState::Menu(UniverseGeneration))
                }

                if ui.button("Settings").kbgp_navigation().clicked() {
                    ui.kbgp_clear_input();
                    ui.kbgp_set_focus_label(FocusLabel::Initial);
                    switch_app_state!(commands, AppState::Menu(Settings))
                }

                // Quit game option
                if ui.button("Quit").kbgp_navigation().clicked() {
                    exit.send(AppExit);
                }
            });
        });
}
