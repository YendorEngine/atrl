use crate::{prelude::*, systems::*};

const MAIN_MENU_WINDOW_SIZE: [f32; 2] = [300.0, 300.0];

pub fn main_menu(
    mut commands: Commands,
    app_settings: AppSettings,
    mut exit: EventWriter<AppExit>,
    mut egui_context: ResMut<EguiContext>,
) {
    let window_size = app_settings.get_window_size();
    let fixed_position = (
        (window_size.x / 2.0) - (MAIN_MENU_WINDOW_SIZE[0] / 2.0),
        (window_size.y / 2.0) - (MAIN_MENU_WINDOW_SIZE[1] / 2.0),
    );

    egui::Window::new("Away Team Roguelike")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .default_size(MAIN_MENU_WINDOW_SIZE)
        .fixed_pos(fixed_position)
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing = (16.0, 16.0).into();
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
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
        });
}
