use crate::prelude::{
    *,
    types::camera::*,
};

#[derive(Resource)]
pub struct LoadedCameras {
    cameras: HashMap<u8, Entity>,
}

impl LoadedCameras {
    pub(crate) fn new() -> Self {
        Self {
            cameras: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, id: u8, entity: Entity) { self.cameras.insert(id, entity); }

    pub(crate) fn get(&self, id: u8) -> Option<Entity> { self.cameras.get(&id).copied() }
}
