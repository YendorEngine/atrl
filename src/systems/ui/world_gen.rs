use crate::{prelude::*, systems::*};

fn generate_seed(seed: &mut String, previous_seed: &mut String) {
    // Generate a random seed
    let mut rng = rand::thread_rng();
    *seed = rng.next_u64().to_string();
    *previous_seed = seed.clone();
}

fn input_with_validation(ui: &mut egui::Ui, seed: &mut String, previous_seed: &mut String) {
    let response = ui.text_edit_singleline(&mut *seed);
    if response.gained_focus() && *previous_seed != *seed {
        *previous_seed = seed.clone();
    }
    if response.lost_focus() && seed.parse::<u64>().is_err() {
        *seed = previous_seed.clone();
    }
}

pub fn world_gen_menu(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut seed: Local<String>,
    mut previous_seed: Local<String>,
) {
    if seed.is_empty() {
        generate_seed(&mut seed, &mut previous_seed)
    }

    egui::Window::new("World Gen")
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("Random Seed");
                input_with_validation(ui, &mut seed, &mut previous_seed);

                if ui.button("Randomize Seed").clicked() {
                    // Generate a random seed
                    generate_seed(&mut seed, &mut previous_seed);
                }

                ui.add_space(10.0);

                if ui.button("Generate").clicked() {
                    // Move to Generate State
                    println!("Seed: {}", *seed);
                }

                ui.separator();

                if ui.button("Return to Main Menu").clicked() {
                    switch_app_state!(commands, AppState::Menu(Main))
                }
            });
        });
}