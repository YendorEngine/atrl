use bevy_egui::egui::{
    epaint::Shadow,
    style::{Margin, WidgetVisuals, Widgets},
    Color32,
    FontFamily::Proportional,
    FontId, Rounding, TextStyle, Vec2,
};
use bevy_egui_kbgp::egui::Stroke;

use crate::prelude::*;

const _BORDER: f32 = 32.;
const PADDING: f32 = 8.;

pub fn get_style() -> egui::Style {
    egui::Style {
        text_styles: [
            (TextStyle::Body, FontId::new(18.0, Proportional)),
            (TextStyle::Small, FontId::new(12.0, Proportional)),
            (TextStyle::Button, FontId::new(18.0, Proportional)),
            (TextStyle::Heading, FontId::new(32.0, Proportional)),
            (TextStyle::Monospace, FontId::new(18.0, Proportional)),
        ]
        .into(),
        spacing: egui::style::Spacing {
            item_spacing: Vec2::new(PADDING, PADDING),
            button_padding: Vec2::new(PADDING, PADDING),
            window_margin: Margin::same(12.0),
            ..Default::default()
        },
        visuals: egui::Visuals {
            dark_mode: true,
            override_text_color: Some(Color32::WHITE),

            widgets: Widgets {
                inactive: WidgetVisuals {
                    rounding: Rounding::same(10.0),
                    ..egui::Visuals::dark().widgets.inactive
                },
                hovered: WidgetVisuals {
                    rounding: Rounding::same(10.0),
                    bg_fill: Color32::from_rgb(19, 117, 184),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    ..egui::Visuals::dark().widgets.hovered
                },
                active: WidgetVisuals {
                    rounding: Rounding::same(10.0),
                    bg_fill: Color32::from_rgb(230, 50, 50),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    ..egui::Visuals::dark().widgets.hovered
                },
                ..egui::Visuals::dark().widgets
            },
            window_rounding: Rounding::same(2.5),
            window_shadow: Shadow {
                extrusion: 0.0,
                color: Default::default(),
            },
            ..egui::Visuals::dark()
        },
        ..Default::default()
    }

    // let mut style = egui::Style::default();
    // style.spacing.button_padding = (PADDING, PADDING).into();
    // style.spacing.item_spacing = (PADDING, PADDING).into();
    // style.text_styles.insert(TextStyle::Heading, FontId {
    //     size: 42.,
    //     family: FontFamily::Proportional,
    // });
    // style.text_styles.insert(TextStyle::Body, FontId {
    //     size: 32.,
    //     family: FontFamily::Proportional,
    // });
    // style.text_styles.insert(TextStyle::Small, FontId {
    //     size: 18.,
    //     family: FontFamily::Proportional,
    // });
    // style.text_styles.insert(TextStyle::Button, FontId {
    //     size: 40.,
    //     family: FontFamily::Proportional,
    // });

    // style
}
