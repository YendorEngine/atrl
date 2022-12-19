use crate::{components::*, prelude::*};

pub fn cleanup_on_exit_splash(mut commands: Commands, q_cleanup: Query<Entity, With<CleanupOnExitSplash>>) {
    for entity in q_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_on_exit_main_menu(
    mut commands: Commands,
    q_cleanup: Query<Entity, With<CleanupOnExitMainMenu>>,
) {
    for entity in q_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_on_exit_game(mut commands: Commands, q_cleanup: Query<Entity, With<CleanupOnExitGame>>) {
    for entity in q_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
