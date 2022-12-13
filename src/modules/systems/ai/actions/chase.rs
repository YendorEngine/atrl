use std::collections::VecDeque;

use crate::{prelude::*, resources::*};

pub fn chase_action<'w, 's>(
    mut commands: Commands,
    player_entity: Res<PlayerEntity>,
    mut map_manager: MapManager,

    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut BigBrainActionState, &mut ChaseActor)>,

    mut blocking_set: ParamSet<(
        Query<'w, 's, &'static BlocksVision>,
        Query<'w, 's, &'static BlocksMovement>,
    )>,

    mut mobs_q: Query<(
        &PositionComponent,
        &FieldOfView,
        &Movement,
        &Vision,
        &Name,
        &mut AIComponent,
    )>,
) {
    // use BigBrainActionState::*;

    // let player_position = match mobs_q.get(player_entity.current()) {
    //     Ok((p, ..)) => p.get(),
    //     Err(err) => {
    //         info!("No player found: {}", err);
    //         return;
    //     },
    // };

    // for (Actor(actor), mut action_state, mut chase) in action_q.iter_mut() {
    //     let Ok((&ai_position, fov, movement,vision, name, mut ai_component)) =
    //         mobs_q.get_mut(*actor) else {
    //             info!("Actor must have required components");
    //             continue;
    //         };

    //     if ai_component.has_action() {
    //         // already chasing, quick return;
    //         continue;
    //     }

    //     match *action_state {
    //         // Success | Failure
    //         Success | Failure | Init => {
    //             info!("{} chase state: {:?}", name, action_state);
    //             ai_component.clear_action();

    //             continue;
    //         },
    //         Cancelled => {
    //             info!("{} cancelled chase!", name);
    //             *action_state = Failure;
    //             ai_component.clear_action();

    //             continue;
    //         },
    //         Requested => {
    //             info!("{} gonna start chasing!", name);
    //             *action_state = Executing;

    //             chase.generated_path = false;
    //             chase.last_seen_pt = Some(player_position);
    //             ai_component.set_action(MovementAction(player_position).boxed());

    //             // Enemy AI chasing the player is cause for alarm!
    //             // Lets stop all input from the player for a short time so they have a
    // chance to react!             commands.init_resource::<UnsafeInput>();

    //             if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
    //                 target_visualizer.set_color(Color::RED);
    //                 target_visualizer.set_style(TargetVisualizerStyle::Target);
    //             }
    //         },
    //         Executing => {},
    //     }

    //     info!("{} executing chase!", name);

    //     let ai_position = ai_position.get();
    //     let position = if entity_in_fov(
    //         &mut map_manager,
    //         &blocking_set.p0(),
    //         fov.0 as u32 + 2,
    //         vision.0,
    //         ai_position,
    //         player_position,
    //     ) {
    //         if in_attack_range(ai_position, player_position) {
    //             info!("{} is in attack range. Moving to attack action", name);
    //             *action_state = Success;
    //             continue;
    //         }

    //         chase.last_seen_pt = Some(player_position);
    //         chase.generated_path = false;
    //         player_position
    //     } else {
    //         let Some(last_seen) = chase.last_seen_pt else {
    //                     error!("Executing chase with no target.");
    //                     ai_component.set_action(WaitAction.boxed());
    //                     continue;
    //                 };

    //         // We reached the end of our chase path and we do not see the player :(
    //         if last_seen == ai_position {
    //             info!("{} lost sight of player and reached end of path", name);
    //             *action_state = Failure;
    //             continue;
    //         }

    //         // We have lost sight of the player and need a path to their last seen position.
    //         // Our pathfinder will only generate a valid path to the last seen location,
    // this         // includes partial path. We can expect the first element in the path
    //         // to be a valid location that is closest to the last_seen_pt.
    //         if !chase.generated_path {
    //             let path = generate_last_seen_path(
    //                 ai_position,
    //                 last_seen,
    //                 movement.0,
    //                 &mut map_manager,
    //                 &blocking_set.p1(),
    //             );

    //             let point = path.first().unwrap_or(&last_seen);

    //             chase.generated_path = true;
    //             chase.last_seen_pt = Some(*point);
    //             *point
    //         } else {
    //             last_seen
    //         }
    //     };

    //     ai_component.set_action(MovementAction(position).boxed());
    // }
}

// fn generate_last_seen_path<'w, 's>(
//     ai_pos: Position,
//     target_pos: Position,
//     movement_type: u8,
//     map_manager: &mut MapManager<'w, 's>,
//     q_blocks_movement: &Query<'w, 's, &'static BlocksMovement>,
// ) -> Vec<Position> {
//     PathFinder::Astar.compute(
//         ai_pos,
//         target_pos,
//         &mut MapPathFinder,
//         PathPassThroughData {
//             map_manager,
//             movement_type,
//             q_blocks_movement,
//         },
//     )
// }
