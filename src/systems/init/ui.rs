use crate::{prelude::*, ui::*};

pub fn init_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
) {
    font_mapping.set_default(asset_server.load("fonts/robo/roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<UIWidget, EmptyState>();
    widget_context.add_widget_system(
        UIWidget::default().get_name(),
        widget_update::<UIWidget, EmptyState>,
        ui_widget_render,
    );

    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            <UIWidgetBundle></UIWidgetBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
