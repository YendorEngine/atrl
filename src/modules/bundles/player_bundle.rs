use crate::prelude::{
    *,
    bundles::*,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub actor: ActorBundle,
    #[bundle]
    pub input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use PlayerAction::*;

        let mut input_map = InputMap::default();

        // Mouse Mapping
        input_map.insert(MouseButton::Left, MouseLeftClick);
        input_map.insert(MouseButton::Right, MouseRightClick);

        /////////////////////////////
        // Movement
        /////////////////////////////
        input_map
            // ArrowKeys
            .insert(KeyCode::Up, North)
            .insert(KeyCode::Down, South)
            .insert(KeyCode::Left, West)
            .insert(KeyCode::Right, East)

            // WSAD
            .insert(KeyCode::W, North)
            .insert(KeyCode::S, South)
            .insert(KeyCode::A, West)
            .insert(KeyCode::D, East)

            // Diagonals
            .insert(KeyCode::Y, NorthWest)
            .insert(KeyCode::U, NorthEast)
            .insert(KeyCode::B, SouthWest)
            .insert(KeyCode::N, SouthEast)

            // GamePad
            .insert(GamepadButtonType::DPadUp, North)
            .insert(GamepadButtonType::DPadDown, South)
            .insert(GamepadButtonType::DPadLeft, West)
            .insert(GamepadButtonType::DPadRight, East);

        input_map
            // Numpad
            .insert(KeyCode::Numpad7, NorthWest)
            .insert(KeyCode::Numpad8, North)
            .insert(KeyCode::Numpad9, NorthEast)
            .insert(KeyCode::Numpad6, East)
            .insert(KeyCode::Numpad3, SouthEast)
            .insert(KeyCode::Numpad2, South)
            .insert(KeyCode::Numpad1, SouthWest)
            .insert(KeyCode::Numpad4, West);

        /////////////////////////////
        // Actions
        /////////////////////////////
        input_map
            // Waiting
            .insert(KeyCode::Period, Wait)
            .insert(KeyCode::Numpad5, Wait);
        input_map
    }
}
