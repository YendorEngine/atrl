use crate::{prelude::*, components::*};

pub fn cleanup_on_enter_main_menu(
    mut commands: Commands,
    q_cleanup: Query<Entity, With<CleanupOnEnterMainMenu>>,
) {
    for entity in q_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_on_enter_game(
    mut commands: Commands,
    q_cleanup: Query<Entity, With<CleanupOnEnterGame>>,
) {
    for entity in q_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
