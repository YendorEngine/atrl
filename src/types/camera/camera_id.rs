#[repr(u8)]
pub enum CameraId {
    Map,
}

impl From<CameraId> for u8 {
    fn from(cam: CameraId) -> Self { cam as Self }
}
