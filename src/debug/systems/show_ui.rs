use bevy_inspector_egui::{bevy_egui, egui};

use crate::prelude::*;

pub fn inspector_ui(
    world: &mut World,
    mut inactive: Local<bool>,
    mut selected_entities: Local<SelectedEntities>,
) {
    let input = world.resource::<Input<KeyCode>>();
    if input.just_pressed(KeyCode::Escape) {
        *inactive = !*inactive;
    }

    if *inactive {
        return;
    }

    let egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();

    egui::SidePanel::left("hierarchy").default_width(200.0).show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Hierarchy");

            bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(world, ui, &mut selected_entities);

            ui.label("Press escape to toggle UI");
            ui.allocate_space(ui.available_size());
        });
    });

    egui::SidePanel::right("inspector").default_width(250.0).show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Inspector");

            let in_header = selected_entities.len() > 1;
            for entity in selected_entities.iter() {
                bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui, in_header);
            }

            ui.allocate_space(ui.available_size());
        });
    });
}
