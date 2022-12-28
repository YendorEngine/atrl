use crate::{
    prelude::*,
    resources::{
        font_storage::FontStorageResource, image_storage::ImageStorageResource,
        tileset_storage::TilesetStorageResource,
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
    // Load tilesets:
    let mut tilesets = vec![
        asset_server.load(TILESET_TINY_GALAXY_FX_PATH),
        asset_server.load(TILESET_TINY_GALAXY_INTERFACE_PATH),
        asset_server.load(TILESET_TINY_GALAXY_ITEMS_PATH),
        asset_server.load(TILESET_TINY_GALAXY_MONSTERS_PATH),
        asset_server.load(TILESET_TINY_GALAXY_PORTRAITS_PATH),
        // TODO: Split SPACE up into multiple tilesets of the same size. `8x8` vs `24x24` etc
        // asset_server.load(TILESET_TINY_GALAXY_SPACE_PATH,
        asset_server.load(TILESET_TINY_GALAXY_WORLD_PATH),
    ];

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
    let tile_handle = TileHandle::new_standard(TILE_WHITE_PIXEL, image_handle.clone());

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

    // Tilesets
    commands.insert_resource(TilesetStorageResource(tilesets));

    // Images
    commands.insert_resource(
        ImageStorageResource::default()
            .insert(image_handle)
            .insert(asset_server.load("images/splash.png"))
            .insert(asset_server.load("images/ui/atrl_logo.png")),
    );

    // Fonts
    commands.insert_resource(
        FontStorageResource::default()
            .load(
                &asset_server,
                "julia_mono_regular",
                "fonts/julia_mono/JuliaMono-Regular.ttf",
            )
            .load(
                &asset_server,
                "julia_mono_bold",
                "fonts/julia_mono/JuliaMono-Bold.ttf",
            )
            .load(
                &asset_server,
                "julia_mono_light",
                "fonts/julia_mono/JuliaMono-Light.ttf",
            ),
    );
}

pub fn wait_for_assets_to_load(mut commands: Commands, tilesets: Tilesets) {
    if ![
        // Fx
        (TILESET_TINY_GALAXY_FX_ID, TILESET_TINY_GALAXY_FX_PATH),
        // Interface
        (
            TILESET_TINY_GALAXY_INTERFACE_ID,
            TILESET_TINY_GALAXY_INTERFACE_PATH,
        ),
        // Items
        (TILESET_TINY_GALAXY_ITEMS_ID, TILESET_TINY_GALAXY_ITEMS_PATH),
        // Monsters
        (
            TILESET_TINY_GALAXY_MONSTERS_ID,
            TILESET_TINY_GALAXY_MONSTERS_PATH,
        ),
        // Portraits
        (
            TILESET_TINY_GALAXY_PORTRAITS_ID,
            TILESET_TINY_GALAXY_PORTRAITS_PATH,
        ),
        // Space
        // TODO: See init_assets
        // if !check_tileset(TILESET_TINY_GALAXY_SPACE_ID, TILESET_TINY_GALAXY_SPACE_PATH) { return; }

        // World
        (TILESET_TINY_GALAXY_WORLD_ID, TILESET_TINY_GALAXY_WORLD_PATH),
        (TILESET_WHITE_PIXEL_ID, TILESET_WHITE_PIXEL),
    ]
    .iter()
    .all(|(id, path)| check_tileset(*id, path, &tilesets))
    {
        return;
    }

    switch_app_state!(commands, AppState::SPLASH_SCREEN_TO_THIS_STATE())
}

fn check_tileset(id: u8, name: &str, tilesets: &Tilesets) -> bool {
    if tilesets.get_by_id(&id).is_some() {
        true
    } else {
        info!("Waiting on tileset ({})...", name);
        false
    }
}
