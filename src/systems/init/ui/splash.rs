use crate::{components::CleanupOnExitSplash, prelude::*};

pub fn init_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            CleanupOnExitSplash,
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
