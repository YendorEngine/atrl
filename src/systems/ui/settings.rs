use bevy_egui::egui::Pos2;

use crate::{prelude::*, systems::*};

const RESOLUTIONS: [(f32, f32); 2] = [(1920., 1080.), (1280., 720.)];

pub fn settings_menu(
    mut commands: Commands,
    mut app_settings: AppSettings,
    mut egui_context: ResMut<EguiContext>,
) {
    let mut full_screen = app_settings.get_fullscreen();

    egui::Window::new("Settings")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .fixed_pos((500.0, 200.0))
        .fixed_size((200.0, 200.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Display Settings");

            ui.allocate_space(egui::Vec2::new(100., 0.0));

            ////////////////////////////
            // Window Size
            ////////////////////////////

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Resolution: ");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", app_settings.get_window_size()))
                        .show_ui(ui, |ui| {
                            let mut current_selected: (f32, f32) = app_settings.get_window_size().into();
                            for resolution in RESOLUTIONS.iter() {
                                let value = ui.selectable_value(
                                    &mut current_selected,
                                    *resolution,
                                    format!("{resolution:?}"),
                                );

                                if value.clicked() {
                                    app_settings.set_window_size((*resolution).into());
                                }
                            }
                        });
                });

                ////////////////////////////
                // Full Screen
                ////////////////////////////

                if ui.checkbox(&mut full_screen, "Full Screen").clicked() {
                    app_settings.set_fullscreen(!app_settings.get_fullscreen());
                }
            });

            ui.separator();

            // ui.horizontal(|ui| {
            //     ui.label("Resolution: ");
            //     egui::ComboBox::from_label("")
            //         .selected_text(format!("{}", app_settings.get_window_size()))
            //         .show_ui(ui, |ui| {
            //             let mut current_selected: (f32, f32) = app_settings.get_window_size().into();
            //             for resolution in RESOLUTIONS.iter() {
            //                 let value = ui.selectable_value(
            //                     &mut current_selected,
            //                     *resolution,
            //                     format!("{resolution:?}"),
            //                 );

            //                 if value.clicked() {
            //                     app_settings.set_window_size((*resolution).into());
            //                 }
            //             }
            //         });
            // });

            // ////////////////////////////
            // // Full Screen
            // ////////////////////////////

            // if ui.checkbox(&mut full_screen, "Full Screen").clicked() {
            //     app_settings.set_fullscreen(!app_settings.get_fullscreen());
            // }

            if ui.button("Back").clicked() {
                switch_app_state!(commands, AppState::Menu(Main))
            }
        });
}
