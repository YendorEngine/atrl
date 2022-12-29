use crate::{prelude::*, systems::init::FocusLabel};

pub fn on_switch_state(mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();
    ctx.kbgp_clear_input();
    ctx.kbgp_set_focus_label(FocusLabel::Initial);
}
