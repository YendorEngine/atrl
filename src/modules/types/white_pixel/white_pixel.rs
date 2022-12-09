use crate::prelude::*;
#[derive(Resource)]
pub struct WhitePixel {
    pub handle: Handle<Image>,
}

#[rustfmt::skip]
const PIXEL_DATA: [u8; 16] = [
    255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8,
];

impl FromWorld for WhitePixel {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<ResMut<Assets<Image>>> = SystemState::new(world);
        let mut images = system_state.get_mut(world);
        let image = Image::new(
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            PIXEL_DATA.to_vec(), // white
            TextureFormat::Rgba32Float,
        );
        Self {
            handle: images.add(image),
        }
    }
}
