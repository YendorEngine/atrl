use crate::{components::*, prelude::*};

pub fn update_camera_dimensions(
    app_settings: AppSettings,
    mut q_game_camera: Query<&mut OrthographicProjection, With<GameCameraTag>>,
) {
    if let Ok(mut projection) = q_game_camera.get_single_mut() {
        let grid_size = app_settings.get_grid_size();
        projection.right = grid_size.x as f32;
        projection.top = grid_size.y as f32;
    }
}
