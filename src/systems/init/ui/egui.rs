use crate::{prelude::*, systems::*};

pub fn init_egui(mut commands: Commands, mut ctx: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "julia_mono".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../../../assets/fonts/julia_mono/JuliaMono-Regular.ttf"
        )),
    );
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "julia_mono".to_owned());

    ctx.ctx_mut().set_fonts(fonts);
    ctx.ctx_mut().set_style(get_style());

    // Keyboard Gamepad Navigation
    commands.insert_resource(KbgpSettings {
        allow_keyboard: true,
        allow_gamepads: true,
        allow_mouse_wheel: true,
        allow_mouse_buttons: true,
        prevent_loss_of_focus: true,
        focus_on_mouse_movement: true,
        disable_default_navigation: true,
        disable_default_activation: true,
        allow_mouse_wheel_sideways: true,
        bindings: {
            bevy_egui_kbgp::KbgpNavBindings::default()
                .with_wasd_navigation()
                .with_key(KeyCode::Return, KbgpNavCommand::Click)
        },
    });
}
