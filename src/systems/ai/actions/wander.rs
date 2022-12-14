use crate::{components::*, prelude::*, resources::*, types::*};

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| Uniform::new_inclusive(3, 10));

pub fn wander_action(
    mut map_manager: MapManager,
    mut ai_context: ResMut<AiContext>,
    q_blocks_movement: Query<&BlocksMovement>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut action_q: Query<(&Actor, &mut BigBrainActionState, &mut Wander)>,
    mut spatial_q: Query<(&PositionComponent, &Movement, &Name, &mut AIComponent)>,
) {
    use BigBrainActionState::*;

    for (Actor(actor), mut action_state, mut wander) in action_q.iter_mut() {
        let rng = ai_context.random.get_prng();

        let Ok((pc, movement,name, mut ai_component)) =
        spatial_q.get_mut(*actor) else {
            info!("Actor must have spatial components");
            continue
            };

        if ai_component.has_action() {
            // already wandering, quick return;
            continue;
        }

        match *action_state {
            // Success | Failure
            Init | Success | Failure => {
                // Nothing to do here
                info!("{} wander state: {:?}", name, action_state);
                continue;
            },
            Cancelled => {
                info!("{} cancelled wander", name);
                *action_state = Failure;
                ai_component.clear_action();

                continue;
            },

            // These two states will fall through to execution
            Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.set_color(Color::YELLOW);
                    target_visualizer.set_style(TargetVisualizerStyle::Select);
                }
            },
            Executing => {},
        }

        info!("{} executing wander!", name);

        let ai_position = pc.position;
        let destination = match std::mem::take(&mut wander.destination) {
            Some(destination) => {
                if ai_position.distance(destination) <= 1 {
                    generate_wander_path(
                        rng,
                        &mut map_manager,
                        ai_position,
                        movement.0,
                        &q_blocks_movement,
                    )
                } else {
                    destination
                }
            },
            None => generate_wander_path(
                rng,
                &mut map_manager,
                ai_position,
                movement.0,
                &q_blocks_movement,
            ),
        };

        wander.destination = Some(destination);
        wander.my_previous_location = ai_position;
        ai_component.set_action(MovementAction(destination).boxed());
    }
}

fn generate_wander_path(
    rng: &mut impl RngCore,
    map: &mut MapManager,
    ai_pos: Position,
    movement_type: u8,
    q_blocks_movement: &Query<&BlocksMovement>,
) -> Position {
    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new(ai_pos, wander_radius);
    let positions = wander_circle.get_positions();

    // Loop through all positions trying to find a place the AI can path to,
    // if we can't find a place, just return the AI's current position
    (0..positions.len())
        .try_for_each(|_| {
            let destination = positions.iter().choose(rng).unwrap_or(&ai_pos);
            if map.can_place_actor(*destination, movement_type, q_blocks_movement) {
                std::ops::ControlFlow::Break(*destination)
            } else {
                std::ops::ControlFlow::Continue(())
            }
        })
        .break_value()
        .unwrap_or(ai_pos)
}
