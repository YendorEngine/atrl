use crate::prelude::*;

pub type CurrentAppState = CurrentState<AppState>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    Initializing,          // Buffer to load plugins
    Loading(LoadingState), // AssetLoading / World / Map generation
    Menu(MenuState),       // Main Menu / Settings Menu / World Gen Menus / Character Creation Menus
    InGame,                // Actually playing the game
    Quit,                  // Graceful quitting
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LoadingState {
    Assets,                 // SplashScreen / Asset Loading
    InitGame,               // Initialize resources for the game
    WorldGen,               // Happens once on new game
    MapGen(MapGenState),    // Generate map terrain
    Ready,                  // On new/load game, alert the player we are ready and await input.
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