use crate::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct TerrainTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct FeatureTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct ItemTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct ActorTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct UiTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct PlayerTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct UiCameraTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct GameCameraTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct ViewPointTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct InputTag;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct CleanupOnEnterMainMenu;

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct CleanupOnEnterGame;

pub(super) fn register_tags(app: &mut App) {
    app.register_type::<TerrainTag>();
    app.register_type::<FeatureTag>();
    app.register_type::<ItemTag>();
    app.register_type::<ActorTag>();
    app.register_type::<UiTag>();
    app.register_type::<PlayerTag>();
    app.register_type::<UiCameraTag>();
    app.register_type::<GameCameraTag>();
    app.register_type::<ViewPointTag>();
    app.register_type::<InputTag>();
    app.register_type::<CleanupOnEnterMainMenu>();
    app.register_type::<CleanupOnEnterGame>();
}