use crate::{prelude::*, resources::*, types::*};

pub fn player_input(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: Local<PlayerTimer>,
    game_settings: Res<GameSettings>,
    mouse_position: Res<MousePosition>,
    check_safe: Option<Res<UnsafeInput>>,
    mut action_queue: ResMut<ActionQueue>,
    mut query: Query<&ActionState<PlayerAction>>,
) {
    // If an event happens which the player should pay attention to,
    // UnsafeInput should be inserted as a resource.
    if check_safe.is_some() {
        // stop all actions
        action_queue.clear_actions();

        // If we already have a timer going
        if let Some(unsafe_timer) = &mut timer.unsafe_delay_timer {
            // tick it
            unsafe_timer.tick(time.delta());

            // if the timer is finished
            if unsafe_timer.finished() {
                // clear the UnsafeInput resource
                commands.remove_resource::<UnsafeInput>();
                // remove the timer
                timer.unsafe_delay_timer = None;
            }
        } else {
            // start a new timer.
            timer.unsafe_delay_timer = Some(Timer::new(game_settings.unsafe_duration(), TimerMode::Once));
        }

        // No input this frame!
        return;
    }

    // Tick timer until duration is met.
    if !timer.input_delay_timer.finished() {
        timer.input_delay_timer.tick(time.delta());
    }

    for action_state in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            action_queue.add_action(WaitAction.boxed());
            println!();
            info!("Player gave input: WAIT");
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) ||
                (action_state.pressed(input_direction) &&
                    action_state.current_duration(input_direction) > game_settings.pressed_duration()) &&
                    timer.input_delay_timer.finished()
            {
                if let Some(direction) = input_direction.direction() {
                    timer.input_delay_timer.reset();
                    // Input is taken as a direction the player wants to move,
                    // We can apply that to the current position when the player
                    // *ACTUALLY* gets to move.
                    action_queue.add_action(MovementDeltaAction(direction.coord()).boxed());

                    println!();
                    info!("Player gave input: MOVE");
                }
            }
        }

        if action_state.just_pressed(PlayerAction::MouseRightClick) {
            info!("Mouse position: {}", *mouse_position);
        }

        if action_state.just_pressed(PlayerAction::MouseLeftClick) {
            let pos = mouse_position.get_mouse_position();
            action_queue.add_action(MovementAction(pos).boxed());
            println!();
            info!("Player gave input: MouseLeftClick");
        }
    }
}
