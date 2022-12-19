use crate::{prelude::*, components::*, types::input::MovementInput};

pub fn init_input(
    mut commands: Commands,
){
    commands.spawn((
        InputTag,
        InputManagerBundle::<MovementInput> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::W, MovementInput::North),
                (KeyCode::A, MovementInput::West),
                (KeyCode::S, MovementInput::South),
                (KeyCode::D, MovementInput::East),
            ])
        }
    ));
}