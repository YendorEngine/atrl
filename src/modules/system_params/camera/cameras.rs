use crate::prelude::{
    *,
    resources::*,
};

#[derive(SystemParam)]
pub struct Cameras<'w, 's> {
    loaded_cameras: Res<'w, LoadedCameras>,
    // this allows us to add `'s` lifetime
    _query: Query<'w, 's, ()>,
}

impl<'w, 's> Cameras<'w, 's> {
    pub fn get_camera_entity<Id: Into<u8>>(&self, id: Id) -> Option<Entity> {
        self.loaded_cameras.get(id.into())
    }
}
