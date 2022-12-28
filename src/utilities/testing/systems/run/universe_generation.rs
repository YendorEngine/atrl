use crate::{
    prelude::*,
    utilities::testing::{systems::functions::*, types::*},
};

pub fn universe_generation_menu_test(
    keys: Res<Input<KeyCode>>,
    mut config: ResMut<TestGenConfig>,
    mut egui_context: ResMut<EguiContext>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        config.show_menu = !config.show_menu;
    }

    if config.seed == 0 {
        config.seed = generate_seed();
        config.generate_stars();
    }

    if config.show_menu {
        egui::Window::new("Test Gen").title_bar(true).resizable(true).collapsible(true).show(
            egui_context.ctx_mut(),
            |ui| {
                let mut changed = false;

                // Generator Type
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        changed |= ui
                            .radio_value(&mut config.selected_type, GeneratorType::Noise, "Noise")
                            .clicked();
                        changed |= ui
                            .radio_value(&mut config.selected_type, GeneratorType::Spiral, "Spiral")
                            .clicked();
                    });
                });

                // Seed
                ui.label(format!("Random Seed: {}", config.seed));
                if ui.button("Reseed").clicked() {
                    // Generate a random seed
                    config.seed = generate_seed();
                    changed = true;
                }

                ui.separator();

                match config.selected_type {
                    GeneratorType::Noise => changed |= noise_settings(ui, &mut config.noise),
                    GeneratorType::Spiral => changed |= spiral_settings(ui, &mut config.spiral),
                }

                ui.separator();

                if ui.button("Generate").kbgp_navigation().clicked() || changed {
                    config.generate_stars();
                }
            },
        );
    }
}

fn noise_settings(ui: &mut egui::Ui, config: &mut NoiseConfig) -> bool {
    let mut changed = false;

    changed |= ui.add(egui::Slider::new(&mut config.top, 0..=100).text("Top")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.left, 0..=100).text("Left")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.right, 0..=100).text("Right")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.bottom, 0..=100).text("Bottom")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.cutoff, 0.0..=1.0).text("Cutoff")).changed();

    changed |= ui.add(egui::Slider::new(&mut config.octaves, 1..=10).text("Octaves")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.frequency, 0.0..=1.0).text("Frequency")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.lacunarity, 0.0..=1.0).text("Lacunarity")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.persistence, 0.0..=1.0).text("Persistence")).changed();

    changed
}

fn spiral_settings(ui: &mut egui::Ui, config: &mut SpiralConfig) -> bool {
    let mut changed = false;

    changed |= ui.checkbox(&mut config.draw_lines, "Draw Lines").clicked();
    changed |= ui.add(egui::Slider::new(&mut config.arm_count, 1..=6).text("Arm Count")).changed();
    changed |= ui.add(egui::Slider::new(&mut config.max_stars, 0..=100).text("Max Stars")).changed();

    changed
}
