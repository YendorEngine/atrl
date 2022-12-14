use crate::{components::*, prelude::*, resources::*, systems::functions::*, types::*};

pub fn attack_action(
    mut commands: Commands,
    player_entity: Res<PlayerEntity>,

    mut target_q: Query<&mut TargetVisualizer>,
    mut mobs_q: Query<(&PositionComponent, &Name, &mut AIComponent)>,
    mut action_q: Query<(&Actor, &mut BigBrainActionState), With<AttackActor>>,
) {
    use BigBrainActionState::*;

    let player_position = match mobs_q.get(player_entity.current()) {
        Ok((p, ..)) => p.get(),
        Err(err) => {
            info!("No player found: {}", err);
            return;
        },
    };

    for (Actor(actor), mut action_state) in action_q.iter_mut() {
        let Ok((ai_position, name, mut ai_component)) =
        mobs_q.get_mut(*actor) else {
            info!("Actor must have required attack components");
            continue
        };

        if ai_component.has_action() {
            // already attacking, quick return;
            continue;
        }

        match *action_state {
            // Success | Failure
            Init | Success | Failure => {
                // Nothing to do here
                info!("{} attack state: {:?}", name, action_state);
                continue;
            },
            Cancelled => {
                info!("{} cancelled attack!", name);
                *action_state = Failure;
                ai_component.clear_action();

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                continue;
            },

            // these final two fall through to logic
            Requested => {
                info!("{} gonna start attacking!", name);
                *action_state = Executing;
                ai_component.set_action(AttackAction(player_position).boxed());

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.set_color(Color::RED);
                    target_visualizer.set_style(TargetVisualizerStyle::Target);
                }
            },
            Executing => {},
        }

        if in_attack_range(ai_position.get(), player_position) {
            println!("{name} is attacking!");
            ai_component.set_action(AttackAction(player_position).boxed());
        } else {
            *action_state = Failure;
            ai_component.set_action(MovementAction(player_position).boxed());
        }
    }
}
