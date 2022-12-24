use bevy_egui_kbgp::egui::vec2;

use crate::{
    prelude::*, resources::universe_generation_settings::UniverseGenerationSettings, systems::*,
    ui::TextEditValidation,
};

fn reseed(local_seed: &mut String, mut universe_settings: &mut UniverseGenerationSettings) {
    // Generate a random seed
    let mut rng = Pcg64::from_entropy();
    let seed = rng.next_u64();
    *local_seed = seed.to_string();
    universe_settings.seed = Some(seed);
}

pub fn universe_gen_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,

    mut local_seed: Local<String>,
    mut universe_settings: ResMut<UniverseGenerationSettings>,
) {
    if local_seed.is_empty() || universe_settings.seed.is_none() {
        reseed(&mut local_seed, &mut universe_settings);
    }

    egui::Window::new("Universe Generation")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .min_width(500.0)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_ui_with_layout(
                vec2(350., 50.),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.horizontal_centered(|ui| {
                        TextEditValidation::new()
                            .with_title("Random Seed")
                            .with_validation_fn(|s| s.parse::<u64>().is_ok())
                            .validation_ui(ui, &mut local_seed)
                            .on_hover_text("Must be a valid u64");

                        if ui.button("Randomize").clicked() {
                            reseed(&mut local_seed, &mut universe_settings);
                        }
                    });
                },
            );

            // UNIVERSE SIZE
            ui.horizontal(|ui| {
                ui.label("Universe Size:");
                egui::ComboBox::from_id_source(0)
                    .width(200.0)
                    .wrap(false)
                    .selected_text(universe_settings.get_universe_display())
                    .show_ui(ui, |ui| {
                        for universe_size in UNIVERSE_SIZES.iter() {
                            let value = ui.selectable_value(
                                &mut universe_settings.universe_size,
                                *universe_size,
                                format!(
                                    "{}: {}x{}",
                                    universe_size.1, universe_size.0.x, universe_size.0.y
                                ),
                            );

                            if value.clicked() {}
                        }
                    });
            });

            // SECTOR SIZE
            ui.allocate_ui_with_layout(
                vec2(350., 50.),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.strong("Sector Size: ");

                        ui.add_enabled(
                            false,
                            egui::DragValue::new(&mut universe_settings.sector_size.x)
                                .speed(1.0)
                                .clamp_range(1..=100)
                                .prefix("Width: "),
                        );
                        ui.add_enabled(
                            false,
                            egui::DragValue::new(&mut universe_settings.sector_size.y)
                                .speed(1.0)
                                .clamp_range(1..=100)
                                .prefix("Height: "),
                        );
                    });
                },
            );

            // SYSTEM SIZE
            ui.allocate_ui_with_layout(
                vec2(425., 50.),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.strong("System Size: ");

                        ui.add_enabled(
                            false,
                            egui::DragValue::new(&mut universe_settings.system_size.x)
                                .speed(1.0)
                                .clamp_range(1..=100)
                                .prefix("Width: "),
                        );
                        ui.add_enabled(
                            false,
                            egui::DragValue::new(&mut universe_settings.system_size.y)
                                .speed(1.0)
                                .clamp_range(1..=100)
                                .prefix("Height: "),
                        );
                    });
                },
            );

            // PLANET SIZE
            ui.horizontal(|ui| {
                ui.label("Planet Size:");
                egui::ComboBox::from_id_source(1)
                    .selected_text(universe_settings.get_planet_display())
                    .show_ui(ui, |ui| {
                        for planet_size in PLANET_SIZES.iter() {
                            let value = ui.selectable_value(
                                &mut universe_settings.universe_size,
                                *planet_size,
                                format!("{}: {}x{}", planet_size.1, planet_size.0.x, planet_size.0.y),
                            );

                            if value.clicked() {}
                        }
                    });
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                if ui.button("Generate").clicked() {
                    // Move to Generate State
                    switch_app_state!(commands, AppState::InGame);
                }

                if ui.button("Return to Main Menu").clicked() {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
