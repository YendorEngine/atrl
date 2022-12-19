use crate::{
    prelude::*, resources::tileset_storage::TilesetStorageResource, systems::*, types::asset_ids::tilesets::*,
};

pub fn init_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = Vec::new();

    let handle = asset_server.load(TINY_GALAXY_WORLD_PATH);
    handles.push(handle);

    // insert whitepixel as tileset:
    // https://github.com/MrGVSV/bevy_tileset/blob/main/examples/dynamic.rs

    commands.insert_resource(TilesetStorageResource(handles));
}

pub fn wait_for_assets_to_load(mut commands: Commands, tilesets: Tilesets) {
    if tilesets.get_by_id(&TINY_GALAXY_WORLD_ID).is_some() {
        switch_app_state!(commands, SPLASH_SCREEN_TO_THIS_STATE)
    } else {
        info!("Waiting on tilesets...");
    }
}
