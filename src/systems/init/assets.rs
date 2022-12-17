use crate::{prelude::*, resources::TilesetStorage, systems::*};

pub fn init_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = Vec::new();

    let handle = asset_server.load(TINY_GALAXY_PATH);
    handles.push(handle);

    commands.insert_resource(TilesetStorage(handles));
}

pub fn wait_for_assets_to_load(mut commands: Commands, tilesets: Tilesets) {
    if tilesets.get_by_id(TINY_GALAXY_ID).is_some() {
        switch_app_state!(commands, AppState::InGame)
    } else {
        println!("Waiting on tilesets...");
    }
}
