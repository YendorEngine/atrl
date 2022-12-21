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
                if app_settings.get_window_mode() == WindowMode::Windowed {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        egui::ComboBox::from_label("Resolution: ")
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
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::ComboBox::from_label("Window Mode: ")
                        .width(300.0)
                        .wrap(false)
                        .selected_text(format!("{:?}", match app_settings.get_window_mode() {
                            WindowMode::Windowed => "Windowed",
                            WindowMode::BorderlessFullscreen => "Borderless Fullscreen",
                            WindowMode::Fullscreen => "Fullscreen",
                            _ => panic!("Unsupported Window Mode"),
                        }))
                        .show_ui(ui, |ui| {
                            let mut current_selected = app_settings.get_window_mode();
                            for mode in WINDOW_MODES.iter() {
                                let value =
                                    ui.selectable_value(&mut current_selected, mode.1, mode.0.to_string());

                                if value.clicked() {
                                    app_settings.set_window_mode(mode.1);
                                }
                            }
                        });
                });

                ui.separator();

                // Back
                if ui.button("Back").kbgp_navigation().kbgp_initial_focus().clicked() {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
