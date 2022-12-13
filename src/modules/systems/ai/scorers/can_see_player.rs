use crate::{prelude::*, resources::*};

pub fn can_see_player(
    player_entity: Res<PlayerEntity>,
    mobs_q: Query<(&PositionComponent, &FieldOfView, &Vision)>,
    mut query: Query<(&Actor, &mut Score, &CanSeePlayer)>,

    mut map_manager: MapManager,
) {
    let Ok((player_position, ..)) = mobs_q.get(player_entity.current()) else {
        error!("No player!");
        return;
    };

    for (Actor(actor), mut score, can_see_player) in query.iter_mut() {
        if *actor == player_entity.current() {
            continue;
        }

        let mut current_score = 0.0;
        if let Ok((ai_position, fov, vision)) = mobs_q.get(*actor) {
            if entity_in_fov(
                &mut map_manager,
                fov.0 as u32,
                vision.0,
                ai_position.position,
                player_position.position,
            ) {
                current_score = can_see_player.score_if_true;
            }
        }

        score.set(current_score);
    }
}
