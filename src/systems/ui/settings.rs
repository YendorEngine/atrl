use bevy_egui_kbgp::egui::RichText;

use crate::{prelude::*, systems::*};

pub fn settings_menu(
    mut commands: Commands,
    mut app_settings: AppSettings,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Settings")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Resolution: ").strong());
                    egui::ComboBox::from_label("")
                        .width(300.0)
                        .wrap(false)
                        .selected_text(format!("{}p", app_settings.get_window_resolution().y))
                        .show_ui(ui, |ui| {
                            let mut current_selected: (f32, f32) =
                                app_settings.get_window_resolution().into();
                            for resolution in RESOLUTIONS.iter() {
                                let value = ui.selectable_value(
                                    &mut current_selected,
                                    resolution.size,
                                    resolution.name.to_string(),
                                );

                                if value.clicked() {
                                    app_settings.set_window_resolution((resolution.size).into());
                                }
                            }
                        });
                });

                ////////////////////////////
                // Full Screen
                ////////////////////////////

                let mut full_screen = app_settings.get_fullscreen();
                if ui.checkbox(&mut full_screen, "Full Screen").kbgp_navigation().clicked() {
                    app_settings.set_fullscreen(!app_settings.get_fullscreen());
                }

                ui.separator();

                // Back
                if ui.button("Back").kbgp_navigation().kbgp_initial_focus().clicked() {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
