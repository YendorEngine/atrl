use crate::{
    components::CleanupOnExitMainMenu, prelude::*, resources::ui_image_storage::UiImageStorageResource,
};

pub fn init_main_menu(mut commands: Commands, textures: Res<UiImageStorageResource>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                    ..default()
                },
                ..default()
            },
            CleanupOnExitMainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    ..default()
                },
                image: UiImage(textures.get_unchecked("main_menu_logo")),
                ..default()
            });
        });
}
