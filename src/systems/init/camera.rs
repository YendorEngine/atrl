use crate::{components::*, prelude::*};

pub fn init_ui_camera(mut commands: Commands, app_settings: AppSettings) {
    let grid_size = app_settings.get_grid_size();

    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                left: 0.0,
                right: grid_size.x as f32,
                bottom: 0.0,
                top: grid_size.y as f32,
                near: 0.0,
                far: 1000.0,
                window_origin: WindowOrigin::BottomLeft,
                scaling_mode: ScalingMode::None,
                scale: 1.0,
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1000.0 - 0.1,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        UiCameraTag,
    ));
}

pub fn cleanup_ui_camera(mut commands: Commands, q_camera: Query<Entity, With<UiCameraTag>>) {
    for camera in q_camera.iter() {
        commands.entity(camera).despawn_recursive();
    }
}

pub fn init_game_camera(mut commands: Commands, app_settings: AppSettings) {
    let grid_size = app_settings.get_grid_size();

    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                left: 0.0,
                right: grid_size.x as f32,
                bottom: 0.0,
                top: grid_size.y as f32,
                near: 0.0,
                far: 1000.0,
                window_origin: WindowOrigin::BottomLeft,
                scaling_mode: ScalingMode::None,
                scale: 1.0,
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1000.0 - 0.1,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        GameCameraTag,
    ));
}

pub fn cleanup_game_camera(mut commands: Commands, q_camera: Query<Entity, With<GameCameraTag>>) {
    for camera in q_camera.iter() {
        commands.entity(camera).despawn_recursive();
    }
}
