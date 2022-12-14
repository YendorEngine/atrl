use crate::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////
// Query Manipulation
//////////////////////////////////////////////////////////////////////////////////////////
/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn();
    }
}
/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_children<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_descendants();
    }
}
/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with_recursive<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}
/// Remove a component type from all entities that have it
pub fn remove_from_all<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}
/// Remove a component type from any entities with some other component
pub fn remove_from_all_with<T: Component, W: Component>(
    mut cmd: Commands,
    q: Query<Entity, (With<T>, With<W>)>,
) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}
