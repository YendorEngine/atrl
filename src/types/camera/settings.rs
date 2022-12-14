use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CameraSettings {
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) bottom: f32,
    pub(crate) top: f32,

    pub(crate) id: Option<u8>,

    pub(crate) scale: f32,
    pub(crate) position: Option<Vec2>,
    pub(crate) scaling_mode: ScalingMode,
    pub(crate) clear_color: Option<Color>,
    pub(crate) viewport: Option<Viewport>,
    pub(crate) window_origin: WindowOrigin,
    pub(crate) render_target: Option<RenderTarget>,
}

impl CameraSettings {
    pub fn new() -> Self { Self::default() }

    pub fn new_dimensions<W: Into<f32>, H: Into<f32>>(width: W, height: H) -> Self {
        let left = 0.0;
        let right = width.into();
        let bottom = 0.0;
        let top = height.into();
        Self {
            left,
            right,
            bottom,
            top,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        }
    }

    pub fn with_rect<L: Into<f32>, R: Into<f32>, B: Into<f32>, T: Into<f32>>(
        mut self,
        left: L,
        right: R,
        bottom: B,
        top: T,
    ) -> Self {
        self.left = left.into();
        self.right = right.into();
        self.bottom = bottom.into();
        self.top = top.into();
        self
    }

    pub fn with_id<Id: Into<u8>>(mut self, id: Id) -> Self {
        self.id = Some(id.into());
        self
    }

    pub const fn with_clear_color(mut self, clear_color: Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    pub const fn with_position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }

    pub const fn with_scaling_mode(mut self, mode: ScalingMode) -> Self {
        self.scaling_mode = mode;
        self
    }

    pub const fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub const fn with_viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = Some(viewport);
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn with_render_target(mut self, render_target: RenderTarget) -> Self {
        self.render_target = Some(render_target);
        self
    }
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            id: None,
            viewport: None,
            render_target: None,
            position: None,
            window_origin: WindowOrigin::Center,
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0,
            clear_color: None,
        }
    }
}

impl From<CameraSettings> for Camera2dBundle {
    fn from(settings: CameraSettings) -> Self {
        let target = settings.render_target.map_or_else(RenderTarget::default, |t| t);
        let near = 0.0;
        let far = 1000.0;
        let transform = match settings.position {
            Some(vec) => Transform::from_translation(vec.extend(far - 0.1)),
            None => {
                let x = (settings.left + settings.right) * 0.5;
                let y = (settings.bottom + settings.top) * 0.5;
                Transform::from_xyz(x, y, far - 0.1)
            },
        };

        let clear_color_config =
            settings.clear_color.map_or(ClearColorConfig::Default, ClearColorConfig::Custom);

        Self {
            camera: Camera {
                viewport: settings.viewport,
                // priority: (),
                // is_active: (),
                // computed: (),
                target,
                // hdr: ()
                ..Default::default()
            },
            // camera_render_graph: (),
            projection: OrthographicProjection {
                left: settings.left,
                right: settings.right,
                bottom: settings.bottom,
                top: settings.top,
                near,
                far,
                window_origin: settings.window_origin,
                scaling_mode: settings.scaling_mode,
                scale: settings.scale,
            },
            // visible_entities: (),
            // frustum: (),
            transform,
            // global_transform: (),
            camera_2d: Camera2d {
                clear_color: clear_color_config,
            },
            // tonemapping: ()
            ..Default::default()
        }
    }
}
