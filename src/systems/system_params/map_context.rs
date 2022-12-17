use crate::{prelude::*, resources::MapManagerResource, types::*};

#[derive(SystemParam)]
pub struct MapContext<'w, 's> {
    map_manager: ResMut<'w, MapManagerResource>,
    q_tile_visible: Query<'w, 's, &'static mut TileVisible>,
}

impl<'w, 's> MapContext<'w, 's> {
    pub fn add_entity(&mut self, position: ChunkPosition, entity: Entity) -> bool {
        let celldata = self.map_manager.chunk_manager_mut().get_mut(position);
        celldata.entities.push(entity);

        true
    }

    pub fn remove_entity(&mut self, position: ChunkPosition, entity: Entity) {
        let celldata = self.map_manager.chunk_manager_mut().get_mut(position);
        celldata.entities.retain(|e| *e != entity);
    }

    pub fn set_visible(&mut self, position: ChunkPosition, layer: MapLayer, visible: bool) {
        let local_position = self.map_manager.chunk_manager().local_position(position);
        let tile_pos = TilePos::new(local_position.x, local_position.y);

        // if let Ok(visibility) = self.q_tile_visible.get_mut(entity) {

        //}
    }
}
