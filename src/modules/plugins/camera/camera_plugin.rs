use crate::prelude::{
    *,
    resources::camera::*,
    systems::camera::*,
    types::camera::*,
};

pub struct CameraPlugin {
    camera_settings: Vec<CameraSettings>,
}

impl CameraPlugin {
    pub fn new(camera_settings: CameraSettings) -> Self {
        Self {
            camera_settings: vec![camera_settings],
        }
    }

    pub fn add_camera(mut self, settings: CameraSettings) -> Self {
        self.camera_settings.push(settings);
        self
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let camera_settings = if self.camera_settings.is_empty() {
            #[cfg(feature = "debug")]
            warn!("CameraPlugin has no settings added. Using default camera settings.");
            vec![CameraSettings::default()]
        } else {
            let mut v = Vec::new();
            for settings in &self.camera_settings {
                v.push(settings.clone());
            }
            v
        };

        let camera_settings_resource = CameraSettingsResource::new(camera_settings);
        app.insert_resource(camera_settings_resource);
        app.add_startup_system(spawn_cameras);
    }
}
