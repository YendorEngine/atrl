use crate::{
    prelude::*, resources::universe_generation_settings::UniverseGenerationSettings, systems::*, ui::*,
};

const DEFAULT_SPACING: egui::Vec2 = vec2(20.0, 10.0);

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
    app_settings: AppSettings,
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

            egui::Grid::new("universe").spacing(DEFAULT_SPACING).show(ui, |ui| {
                // Universe Size
                ui.label("Universe Size:");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    for universe_descriptor in UNIVERSE_SIZES.iter() {
                        ui.selectable_value(
                            &mut universe_settings.universe_descriptor,
                            *universe_descriptor,
                            universe_descriptor.to_display(),
                        )
                        .kbgp_navigation();
                    }
                });

                ui.end_row();

                // SECTOR SIZE
                ui.strong("Sector Size: ");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
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
                ui.end_row();

                ui.strong("System Size: ");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
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
                ui.end_row();

                // PLANET SIZE
                ui.label("Planet Size:");
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        egui::ComboBox::from_id_source("planet_size")
                            .width(300.0)
                            .selected_text(universe_settings.planet_descriptor.to_display())
                            .show_ui(ui, |ui| {
                                for planet_descriptor in PLANET_SIZES.iter() {
                                    ui.selectable_value(
                                        &mut universe_settings.planet_descriptor,
                                        *planet_descriptor,
                                        planet_descriptor.to_display(),
                                    )
                                    .kbgp_navigation();
                                }
                            })
                            .response
                            .kbgp_navigation();
                    },
                );
                ui.end_row();
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                if ui.button("Generate").kbgp_navigation().clicked() {
                    // set global contants
                    set_universe_sizes(&universe_settings);

                    // dummy generate star system. remove later
                    let grid_size = app_settings.get_grid_size();
                    universe_settings.generate_noise(grid_size);

                    switch_app_state!(commands, AppState::InGame);
                }

                if ui.button("Return to Main Menu").kbgp_navigation().clicked() ||
                    ui.kbgp_user_action() == Some(KbgpUserAction::Back)
                {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}
