use crate::{components::CleanupOnExitMainMenu, prelude::*};

pub fn init_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                image: UiImage(asset_server.load("images/splash.png")),
                ..default()
            });
        });
}
