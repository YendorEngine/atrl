use crate::prelude::{
    *,
    resources::*,
};

pub fn spawn_cameras(mut commands: Commands, mut camera_settings: ResMut<CameraSettingsResource>) {
    let mut loaded_cameras = LoadedCameras::new();

    for settings in camera_settings.settings.drain(..) {
        let entity = commands.spawn_empty().id();
        settings.id.map_or_else(
            || {
                info!("Spawning camera: [NO ID]");
            },
            |id| {
                info!("Spawning camera: [{}]", id);
                loaded_cameras.add(id, entity)
            },
        );
        commands.entity(entity).insert(Camera2dBundle::from(settings));
    }

    commands.remove_resource::<CameraSettingsResource>();
    commands.insert_resource(loaded_cameras);
}
