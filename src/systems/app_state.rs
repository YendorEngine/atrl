use crate::prelude::*;

pub type CurrentAppState = CurrentState<AppState>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    Initializing, // Buffer to load plugins
    Loading,
    Menu(MenuState), // Main Menu / Settings Menu / World Gen Menus / Character Creation Menus
    InGame,          // Actually playing the game
    Quit,            // Graceful quitting
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MapGenState {
    Terrain,
    Features,
    Items,
    Actors,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuState {
    MainMenu,
    Settings,
    WorldCreation(WorldCreationState),
    CharacterCreation(CharacterCreationState),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WorldCreationState {
    Screen1, // Seed etc
    Screen2, // Seed etc
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CharacterCreationState {
    RaceClass,  // Race / Class
    Attributes, // Attributes
    Skills,     // Skills
    Feats,      // Feats
}
