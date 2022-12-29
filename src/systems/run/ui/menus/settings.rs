use crate::{prelude::*, systems::*, types::resolution::*};

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
                            .selected_text(format!("{}", app_settings.get_window_resolution()))
                            .show_ui(ui, |ui| {
                                let mut current_resolution = app_settings.get_window_resolution();
                                for resolution in Resolution::iter() {
                                    if ui
                                        .selectable_value(
                                            &mut current_resolution,
                                            *resolution,
                                            resolution.to_string(),
                                        )
                                        .kbgp_navigation()
                                        .clicked()
                                    {
                                        app_settings.set_window_resolution(*resolution);
                                    }
                                }
                            })
                            .response
                            .kbgp_navigation()
                            .kbgp_initial_focus()
                            .kbgp_focus_label(FocusLabel::Initial);
                    });
                }

                // if let Resolution::Custom(_, _) = app_settings.get_window_resolution() {
                //     let mut changed = false;

                //     ui.horizontal(|ui| {
                //         ui.label("Width: ");
                //         changed |= ui.text_edit_singleline(&mut *custom_x).changed()
                //     });

                //     ui.horizontal(|ui| {
                //         ui.label("Height: ");
                //         changed |= ui.text_edit_singleline(&mut *custom_y).changed()
                //     });

                //     if changed {
                //         app_settings.set_window_resolution(Resolution::Custom(
                //             custom_x.parse::<f32>().unwrap(),
                //             custom_y.parse::<f32>().unwrap(),
                //         ));
                //     }
                // }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::ComboBox::from_label("Window Mode: ")
                        .width(300.0)
                        .wrap(false)
                        .selected_text(format!("{:?}", match app_settings.get_window_mode() {
                            WindowMode::Windowed => "Windowed",
                            WindowMode::BorderlessFullscreen => "Borderless Fullscreen",
                            _ => panic!("Unsupported Window Mode"),
                        }))
                        .show_ui(ui, |ui| {
                            let mut current_selected = app_settings.get_window_mode();
                            for window_descriptor in WINDOW_DESCRIPTORS.iter() {
                                let window_mode = window_descriptor.get_data();
                                let window_name: String = window_descriptor.into();

                                let value = ui
                                    .selectable_value(&mut current_selected, window_mode, window_name)
                                    .kbgp_navigation();

                                if value.clicked() {
                                    app_settings.set_window_mode(window_mode);
                                }
                            }
                        })
                        .response
                        .kbgp_navigation()
                        .kbgp_initial_focus()
                        .kbgp_focus_label(FocusLabel::Initial);
                });

                ui.separator();

                // Back
                if ui.button("Back").kbgp_navigation().clicked() ||
                    ui.kbgp_user_action() == Some(KbgpUserAction::Back)
                {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
