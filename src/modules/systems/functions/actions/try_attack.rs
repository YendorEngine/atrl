use crate::prelude::*;

pub fn try_attack(entity: Entity, position: Position, world: &mut World) -> Result<(), BoxedAction> {
    let mut system_state: SystemState<(
        MapManager,
        Query<(&mut Health, &Name)>,
        // EventWriter<EffectType>,
    )> = SystemState::new(world);
    let (mut map_manager, mut health_q) = system_state.get_mut(world);

    let mut actors = Vec::new();
    let mut features = Vec::new();

    if let Some(victims) = map_manager.get_actors(position) {
        actors = victims.clone();
    }

    if let Some(victims) = map_manager.get_features(position) {
        features = victims.clone();
    }

    let mut has_attacked = false;
    for victim in actors.iter().chain(features.iter()) {
        if let Ok((mut health, name)) = health_q.get_mut(*victim) {
            has_attacked = true;
            let before = format!("{}/{}", health.current_hp, health.max_hp);
            health.current_hp -= 1;
            let after = format!("{}/{}", health.current_hp, health.max_hp);
            println!("{name} is attacking {entity:?} before: ({before}) after: ({after})");
            // effects_writer.send(EffectType::Damage(1));
        }
    }

    if has_attacked {
        Ok(())
    } else {
        info!("Couldn't find entities with health components.");
        Err(WaitAction.boxed())
    }
}
