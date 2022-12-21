use crate::{
    prelude::*,
    resources::{
        font_storage::FontStorageResource, image_storage::ImageStorageResource,
        tileset_storage::TilesetStorageResource, ui_image_storage::UiImageStorageResource,
    },
    systems::*,
    types::asset_ids::tilesets::*,
};

#[rustfmt::skip]
const PIXEL_DATA: [u8; 4] = [
    255u8, 255u8, 255u8, 255u8,
];

pub fn init_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_image: ResMut<Assets<Image>>,
    mut asset_texture: ResMut<Assets<TextureAtlas>>,
    mut asset_tilesets: ResMut<Assets<Tileset>>,
) {
    let mut tilesets = Vec::new();
    let mut images = Vec::new();

    // Load tilesets:
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_FX_PATH));
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_INTERFACE_PATH));
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_ITEMS_PATH));
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_MONSTERS_PATH));
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_PORTRAITS_PATH));
    // TODO: Split SPACE up into multiple tilesets of the same size. `8x8` vs `24x24` etc
    // tilesets.push(asset_server.load(TILESET_TINY_GALAXY_SPACE_PATH));
    tilesets.push(asset_server.load(TILESET_TINY_GALAXY_WORLD_PATH));

    // Load white_pixel:
    let image = Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        PIXEL_DATA.to_vec(),
        TextureFormat::Rgba8UnormSrgb,
    );
    let image_handle = asset_image.add(image);
    images.push(image_handle.clone());
    let tile_handle = TileHandle::new_standard(TILE_WHITE_PIXEL, image_handle);

    let mut builder = TilesetBuilder::default();
    if builder.add_tile(tile_handle, TILE_WHITE_PIXEL_ID, &asset_image).is_ok() {
        if let Ok(raw_tileset) = builder.build(
            TILESET_WHITE_PIXEL,
            TILESET_WHITE_PIXEL_ID,
            &mut asset_image,
        ) {
            let tileset_handle = asset_tilesets.add(raw_tileset.into_asset(&mut asset_texture));
            tilesets.push(tileset_handle);
        }
    }

    // insert whitepixel as tileset:
    // https://github.com/MrGVSV/bevy_tileset/blob/main/examples/dynamic.rs

    commands.insert_resource(TilesetStorageResource(tilesets));
    commands.insert_resource(ImageStorageResource(images));

    // Textures
    commands.insert_resource(
        UiImageStorageResource::default()
            .insert("splash".to_string(), asset_server.load("images/splash.png"))
            .insert(
                "main_menu_logo".to_string(),
                asset_server.load("images/ui/atrl_logo.png"),
            ),
    );

    // Fonts
    commands.insert_resource(
        FontStorageResource::default()
            .insert(
                "julia_mono".to_string(),
                asset_server.load("fonts/julia_mono/JuliaMono-Regular.ttf"),
            )
            .insert(
                "julia_mono_bold".to_string(),
                asset_server.load("fonts/julia_mono/JuliaMono-Bold.ttf"),
            )
            .insert(
                "julia_mono_light".to_string(),
                asset_server.load("fonts/julia_mono/JuliaMono-Light.ttf"),
            ),
    );
}

pub fn wait_for_assets_to_load(mut commands: Commands, tilesets: Tilesets, time: Res<Time>) {
    if !check_tileset(
        TILESET_TINY_GALAXY_FX_ID,
        TILESET_TINY_GALAXY_FX_PATH,
        &tilesets,
    ) {
        return;
    }
    if !check_tileset(
        TILESET_TINY_GALAXY_INTERFACE_ID,
        TILESET_TINY_GALAXY_INTERFACE_PATH,
        &tilesets,
    ) {
        return;
    }
    if !check_tileset(
        TILESET_TINY_GALAXY_ITEMS_ID,
        TILESET_TINY_GALAXY_ITEMS_PATH,
        &tilesets,
    ) {
        return;
    }
    if !check_tileset(
        TILESET_TINY_GALAXY_MONSTERS_ID,
        TILESET_TINY_GALAXY_MONSTERS_PATH,
        &tilesets,
    ) {
        return;
    }
    if !check_tileset(
        TILESET_TINY_GALAXY_PORTRAITS_ID,
        TILESET_TINY_GALAXY_PORTRAITS_PATH,
        &tilesets,
    ) {
        return;
    }
    // TODO: See init_assets
    // if !check_tileset(TILESET_TINY_GALAXY_SPACE_ID, TILESET_TINY_GALAXY_SPACE_PATH) { return; }
    if !check_tileset(
        TILESET_TINY_GALAXY_WORLD_ID,
        TILESET_TINY_GALAXY_WORLD_PATH,
        &tilesets,
    ) {
        return;
    }
    if !check_tileset(TILESET_WHITE_PIXEL_ID, TILESET_WHITE_PIXEL, &tilesets) {
        return;
    }

    switch_app_state!(commands, SPLASH_SCREEN_TO_THIS_STATE)
}

fn check_tileset(id: u8, name: &str, tilesets: &Tilesets) -> bool {
    if tilesets.get_by_id(&id).is_some() {
        true
    } else {
        info!("Waiting on tileset ({})...", name);
        false
    }
}
