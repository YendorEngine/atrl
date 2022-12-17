use crate::{prelude::*, systems::functions::create_tilemap, types::*};

#[derive(Resource)]
pub struct MapManagerResource {
    chunk_manager: ChunkManager<CellData>,
    terrain_map_entity: Entity,
    feature_map_entity: Entity,
    item_map_entity: Entity,
    actor_map_entity: Entity,
    ui_map_entity: Entity,
}

impl FromWorld for MapManagerResource {
    fn from_world(world: &mut World) -> Self {
        let mut state: SystemState<(Commands, Tilesets, AppSettings)> = SystemState::new(world);
        let (mut commands, tilesets, app_settings) = state.get_mut(world);
        let dimensions = app_settings.get_grid_size();

        Self {
            chunk_manager: ChunkManager::new(dimensions, MyChunkProvider {}),
            terrain_map_entity: create_tilemap(
                &mut commands,
                &tilesets,
                &app_settings,
                f32::from(MapLayer::Terrain),
            ),
            feature_map_entity: create_tilemap(
                &mut commands,
                &tilesets,
                &app_settings,
                f32::from(MapLayer::Features),
            ),
            item_map_entity: create_tilemap(
                &mut commands,
                &tilesets,
                &app_settings,
                f32::from(MapLayer::Items),
            ),
            actor_map_entity: create_tilemap(
                &mut commands,
                &tilesets,
                &app_settings,
                f32::from(MapLayer::Actors),
            ),
            ui_map_entity: create_tilemap(
                &mut commands,
                &tilesets,
                &app_settings,
                f32::from(MapLayer::Ui),
            ),
        }
    }
}

impl MapManagerResource {
    pub fn chunk_manager(&self) -> &ChunkManager<CellData> { &self.chunk_manager }

    pub fn chunk_manager_mut(&mut self) -> &mut ChunkManager<CellData> { &mut self.chunk_manager }

    pub fn terrain_map(&self) -> Entity { self.terrain_map_entity }

    pub fn feature_map(&self) -> Entity { self.feature_map_entity }

    pub fn item_map(&self) -> Entity { self.item_map_entity }

    pub fn actor_map(&self) -> Entity { self.actor_map_entity }

    pub fn ui_map(&self) -> Entity { self.ui_map_entity }

    // pub fn get_terrain_tile_index(&self, position: ChunkPosition) -> u32 {
    //    let celldata = self.chunk_manager.get_mut(position);
    //    celldata.terrain_tile_index
    //}
    // pub fn get_feature_tile_index(&self, position: ChunkPosition) -> Option<u32> {
    //    let celldata = self.chunk_manager.get_mut(position);
    //    celldata.feature_tile_index
    //}
    // pub fn get_item_tile_index(&self, position: ChunkPosition) -> Option<u32> {
    //    let celldata = self.chunk_manager.get_mut(position);
    //    celldata.item_tile_index
    //}
    // pub fn get_actor_tile_index(&self, position: ChunkPosition) -> Option<u32> {
    //    let celldata = self.chunk_manager.get_mut(position);
    //    celldata.actor_tile_index
    //}
}

pub struct CellData {
    pub entities: Vec<Entity>,
    pub terrain_tile_index: u32,
    pub item_tile_index: Option<u32>,
    pub actor_tile_index: Option<u32>,
    pub feature_tile_index: Option<u32>,
}

pub struct MyChunkProvider;

impl ChunkProvider<CellData> for MyChunkProvider {
    fn load(&mut self, _world_position: ChunkWorldPosition, dimensions: ChunkDimensions) -> Vec<CellData> {
        let capacity = (dimensions.x * dimensions.y) as usize;
        let mut cells = Vec::with_capacity(capacity);
        for _y in 0..dimensions.y {
            for _x in 0..dimensions.x {
                cells.push(CellData {
                    entities: Vec::new(),
                    item_tile_index: None,
                    actor_tile_index: None,
                    feature_tile_index: None,
                    terrain_tile_index: TG_WORLD_BARREL_ID,
                });
            }
        }
        cells
    }

    fn store(&mut self, _world_position: ChunkWorldPosition, _cells: Vec<CellData>) {}
}
