use crate::{
    components::{CleanupOnExitMainMenu, MainBackgroundTag},
    prelude::*,
};

pub fn init_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_background: Query<Entity, With<MainBackgroundTag>>,
) {
    if !q_background.is_empty() {
        println!("Main menu already initialized");
        return;
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            CleanupOnExitMainMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage(asset_server.load("images/splash.png")),
                    ..default()
                },
                MainBackgroundTag,
            ));
        });
}
