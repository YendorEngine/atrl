use crate::prelude::*;

#[derive(Resource)]
pub struct ImageStorageResource(pub Vec<Handle<Image>>);
