use crate::{prelude::*, resources::*};

pub fn try_move(world: &mut World, entity: Entity, destination: Position) -> Result<(), BoxedAction> {
    Err(WaitAction.boxed())
    // let mut system_state: SystemState<(
    //     MapManager,
    //     Query<(&mut Position, &Movement)>,
    //     Query<&BlocksMovement>,
    //     Res<PlayerEntity>,
    // )> = SystemState::new(world);
    // let (mut map_manager, mut spatial_q, q_blocks_movement, player) =
    // system_state.get_mut(world);

    // spatial_q.get_mut(entity).map_or_else(
    //     |err| {
    //         info!("Couldn't find entities position components: {}", err);
    //         Err(WaitAction.boxed())
    //     },
    //     |(mut from_position, movement_component)| {
    //         if **player == entity {
    //             Ok(destination)
    //         } else {
    //             PathFinder::Astar
    //                 .compute(
    //                     *from_position,
    //                     destination,
    //                     &mut MapPathFinder,
    //                     PathPassThroughData {
    //                         map_manager: &mut map_manager,
    //                         movement_type: movement_component.0,
    //                         q_blocks_movement: &q_blocks_movement,
    //                     },
    //                 )
    //                 .pop()
    //                 .map_or_else(
    //                     || {
    //                         info!("Couldn't find a long enough path to {:?}", destination);
    //                         Err(WaitAction.boxed())
    //                     },
    //                     Ok,
    //                 )
    //         }
    //         .map_or_else(Err, |destination| {
    //             if map_manager.move_actor(
    //                 entity,
    //                 *from_position,
    //                 destination,
    //                 movement_component.0,
    //                 &q_blocks_movement,
    //             ) {
    //                 *from_position = destination;
    //                 Ok(())
    //             } else {
    //                 info!("{:?} is blocked!", destination);
    //                 Err(WaitAction.boxed())
    //             }
    //         })
    //     },
    // )
}
