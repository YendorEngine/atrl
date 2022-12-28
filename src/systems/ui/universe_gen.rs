use crate::{
    prelude::*, resources::universe_generation_settings::UniverseGenerationSettings, systems::*, ui::*,
};

#[derive(Deref, DerefMut)]
pub struct LocalSeed(pub ValText<u64>);

impl Default for LocalSeed {
    fn default() -> Self { Self(ValText::with_validator(|s| s.parse::<u64>().ok())) }
}

fn reseed(local_seed: &mut Local<LocalSeed>, mut universe_settings: &mut UniverseGenerationSettings) {
    // Generate a random seed
    let mut rng = Pcg64::from_entropy();
    let seed = rng.next_u64();
    local_seed.set_val(seed);
    universe_settings.seed = Some(seed);
}

pub fn universe_gen_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,

    mut local_seed: Local<LocalSeed>,
    mut universe_settings: ResMut<UniverseGenerationSettings>,
) {
    if local_seed.is_empty() || universe_settings.seed.is_none() {
        reseed(&mut local_seed, &mut universe_settings);
    }

    let ctx = egui_context.ctx_mut();
    egui::Window::new("Universe Generation")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .min_width(800.0)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // RANDOM SEED
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 50.),
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.horizontal_centered(|ui| {
                        OptionEditor::new("Seed", &mut **local_seed, |seed| println!("{seed}"))
                            .allow_invalid(false)
                            .ui(ui);

                        if ui
                            .button("Randomize")
                            .kbgp_navigation()
                            .kbgp_initial_focus()
                            .kbgp_focus_label(FocusLabel::Initial)
                            .clicked()
                        {
                            reseed(&mut local_seed, &mut universe_settings);
                        }
                    })
                },
            );

            // UNIVERSE SIZE
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 50.),
                egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
                |ui| {
                    ui.label("Planet Size:");

                    egui::Grid::new("universe").num_columns(3).show(ui, |ui| {
                        for (i, universe_size) in UNIVERSE_SIZES.iter().enumerate() {
                            ui.selectable_value(
                                &mut universe_settings.universe_size,
                                *universe_size,
                                format!(
                                    "{}: {}x{}",
                                    universe_size.1, universe_size.0.x, universe_size.0.y
                                ),
                            )
                            .kbgp_navigation();

                            if i != 0 && i % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                },
            );

            // SECTOR SIZE
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 50.),
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
                vec2(ui.available_width(), 50.),
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
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 50.),
                egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
                |ui| {
                    ui.label("Planet Size:");

                    egui::Grid::new("planet").num_columns(3).show(ui, |ui| {
                        for (i, planet_size) in PLANET_SIZES.iter().enumerate() {
                            ui.selectable_value(
                                &mut universe_settings.planet_size,
                                *planet_size,
                                format!("{}: {}x{}", planet_size.1, planet_size.0.x, planet_size.0.y),
                            )
                            .kbgp_navigation();

                            if i != 0 && i % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });
                },
            );

            ui.separator();

            ui.vertical_centered(|ui| {
                if ui.button("Generate").kbgp_navigation().clicked() {
                    // Move to Generate State
                    ctx.kbgp_clear_input();
                    switch_app_state!(commands, AppState::InGame);
                }

                if ui.button("Return to Main Menu").kbgp_navigation().clicked() ||
                    ui.kbgp_user_action() == Some(KbgpUserAction::Back)
                {
                    ctx.kbgp_clear_input();
                    ui.kbgp_set_focus_label(FocusLabel::Initial);
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
