use crate::{prelude::*, systems::*};

pub fn main_menu(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Main Menu")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .default_size((500.0, 500.0))
        .fixed_pos((500.0, 200.0))
        .show(egui_context.ctx_mut(), |ui| {
            if ui.button("Start").clicked() {
                switch_app_state!(commands, AppState::InGame)
            }

            if ui.button("Settings").clicked() {
                switch_app_state!(commands, AppState::Menu(Settings))
            }

            // Quit game option
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
        });
}
