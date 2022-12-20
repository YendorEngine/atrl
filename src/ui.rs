pub mod render {
    mod main_menu_widget;
    pub use main_menu_widget::*;
    mod settings_widget;
    pub use settings_widget::*;
    mod splash_render;
    pub use splash_render::*;
    mod ui_render;
    pub use ui_render::*;
}
pub use render::*;

pub mod widgets {
    mod main_menu_widget;
    pub use main_menu_widget::*;
    mod settings_widget;
    pub use settings_widget::*;
    mod splash_widget;
    pub use splash_widget::*;
    mod ui_widget;
    pub use ui_widget::*;
}
pub use widgets::*;
