use crate::prelude::*;

#[derive(Component, Default, PartialEq, Clone, Debug)]
pub struct UIWidget {
    pub show_ui: bool,
}

impl Widget for UIWidget {}

#[derive(Bundle)]
pub struct UIWidgetBundle {
    pub ui: UIWidget,
    pub styles: KStyle,
    pub widget_name: WidgetName,
}

impl Default for UIWidgetBundle {
    fn default() -> Self {
        Self {
            styles: KStyle::default(),
            ui: UIWidget { show_ui: true },
            widget_name: UIWidget::default().get_name(),
        }
    }
}
