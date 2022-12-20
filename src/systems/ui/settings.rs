use crate::{prelude::*, systems::*};

const SETTINGS_WINDOW_SIZE: [f32; 2] = [300.0, 300.0];

pub fn settings_menu(
    mut commands: Commands,
    mut app_settings: AppSettings,
    mut egui_context: ResMut<EguiContext>,
) {
    let window_size = app_settings.get_window_size();
    let fixed_position = (
        (window_size.x / 2.0) - (SETTINGS_WINDOW_SIZE[0] / 2.0),
        (window_size.y / 2.0) - (SETTINGS_WINDOW_SIZE[1] / 2.0),
    );

    egui::Window::new("Settings")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .default_size(SETTINGS_WINDOW_SIZE)
        .fixed_pos(fixed_position)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Display Settings");

            ui.style_mut().spacing.item_spacing = (16.0, 16.0).into();
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Resolution: ");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}p", app_settings.get_window_size().y))
                        .show_ui(ui, |ui| {
                            let mut current_selected: (f32, f32) = app_settings.get_window_size().into();
                            for resolution in RESOLUTIONS.iter() {
                                let value = ui.selectable_value(
                                    &mut current_selected,
                                    resolution.size,
                                    resolution.name.to_string(),
                                );

                                if value.clicked() {
                                    app_settings.set_window_size((resolution.size).into());
                                }
                            }
                        });
                });

                ////////////////////////////
                // Full Screen
                ////////////////////////////

                let mut full_screen = app_settings.get_fullscreen();
                if ui.checkbox(&mut full_screen, "Full Screen").clicked() {
                    app_settings.set_fullscreen(!app_settings.get_fullscreen());
                }

                ui.separator();

                // Back
                if ui.button("Back").clicked() {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
