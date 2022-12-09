use crate::prelude::{
    *,
    resources::*,
    systems::*,
};

/// MapManager SystemParam used for interacting with the maps.
#[derive(SystemParam)]
pub struct MapManager<'w, 's> {
    commands: Commands<'w, 's>,
    game_context: ResMut<'w, GameContext>,
    map_manager: ResMut<'w, MapManagerResource>,
}

// Perform actor functions on maps
impl<'w, 's> MapManager<'w, 's> {
    pub fn is_visible(&self, position: Position) -> bool {
        self.map_manager.visible_tiles.get_visible(position)
    }

    /// Works the same as `add_actor()` / `move_actor()` without
    /// actually changing anything. Mainly for use in algorithms.
    /// If you are planning on adding/moving the actor after, just
    /// use the `add_actor` or `move_actor` functions.
    ///
    /// Returns `true` if the actor can be placed at that `Position`.
    pub fn can_place_actor(
        &mut self,
        position: Position,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        let Some(map) = self.get_map(position.get_world_position()) else { return false; };

        map.can_place_actor(
            position.get_local_position(),
            movement_type,
            q_blocks_movement,
        )
    }

    /// Attempts to add the actor to the map at a specific `Position`.
    /// This is used when generating a new actor to be placed on the map.
    /// If you want to move an actor, use `move_actor()`.
    ///
    /// Returns `true` if the actor was placed at that `Position`.
    pub fn add_actor(
        &mut self,
        actor: Entity,
        position: Position,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        let Some(map) = self.get_map(position.get_world_position()) else { return false; };

        map.add_actor(
            actor,
            position.get_local_position(),
            movement_type,
            q_blocks_movement,
        )
    }

    /// Attempts to move the actor from one `Position` to another.
    /// If the actor is being generated, use `add_actor()` first.
    ///
    /// Returns `true` if the actor was moved to that `Position`.
    pub fn move_actor(
        &mut self,
        actor: Entity,
        from_position: Position,
        to_position: Position,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        // Try adding the actor to the new Position
        if self.add_actor(actor, to_position, movement_type, q_blocks_movement) {
            // Remove the actor from the old Position
            self.remove_actor(actor, from_position);
            // Everything went good
            true
        } else {
            // can't move the actor to the new Position
            false
        }
    }

    /// Attempts to remove the actor from a `Position`.
    /// This is used when an actor dies. If you want to
    /// move an actor, use `move_actor()`
    ///
    /// Returns `Some(actor_entity)` if the actor was removed.
    pub fn remove_actor(&mut self, actor: Entity, position: Position) {
        let Some(map) = self.get_map(position.get_world_position()) else { return; };

        map.remove_actor(actor, position.get_local_position());
    }

    /// Attempts to get a list of all actors at a `Position`
    ///
    /// Returns `Option<&Vec<Entity>>` if there are actors at the `Position`.
    pub fn get_actors(&mut self, position: Position) -> Option<&Vec<Entity>> {
        let Some(map) = self.get_map(position.get_world_position()) else { return None; };

        map.get_actors(position.get_local_position())
    }
}

// Perform feature functions on maps
impl<'w, 's> MapManager<'w, 's> {
    /// Attempts to add the feature to the map at a specific `Position`.
    /// This is used when generating a new feature to be placed on the map.
    /// If you want to move an feature, use `move_feature()`.
    ///
    /// Returns `true` if the feature was placed at that `Position`.
    pub fn add_feature(&mut self, feature: Entity, position: Position) -> bool {
        let Some(map) = self.get_map(position.get_world_position()) else { return false; };

        map.add_feature(feature, position.get_local_position())
    }

    /// Attempts to move the feature from one `Position` to another.
    /// If the feature is being generated, use `add_feature()` first.
    ///
    /// Returns `true` if the feature was moved to that `Position`.
    pub fn move_feature(&mut self, feature: Entity, from_position: Position, to_position: Position) -> bool {
        // Try adding the feature to the new Position
        if self.add_feature(feature, to_position) {
            // Try removing the feature from the old Position
            self.remove_feature(feature, from_position);
            // Everything went good
            true
        } else {
            // can't move the feature to the new Position
            false
        }
    }

    /// Attempts to remove the feature from a `Position`.
    /// This is used when an feature dies. If you want to
    /// move an feature, use `move_feature()`
    ///
    /// Returns `Some(feature_entity)` if the feature was removed.
    pub fn remove_feature(&mut self, feature: Entity, position: Position) {
        let Some(map) = self.get_map(position.get_world_position()) else { return; };

        map.remove_feature(feature, position.get_local_position());
    }

    /// Attempts to get a list of all features at a `Position`
    ///
    /// Returns `Option<&Vec<Entity>>` if there are features at the `Position`.
    pub fn get_features(&mut self, position: Position) -> Option<&Vec<Entity>> {
        let Some(map) = self.get_map(position.get_world_position()) else { return None; };

        map.get_features(position.get_local_position())
    }
}

//// Perform item functions on maps
// impl<'w, 's> MapManager<'w, 's> {
//    pub fn has_item(&self, position: Position) -> bool {
//        false
//    }
//
//    pub fn list_items(&self, position: Position) -> Option<&ItemStack> {
//        None
//    }
//
//    pub fn add_item(&mut self, item: Entity, position: Position) -> bool {
//        false
//    }
//
//    pub fn remove_item(&mut self, item: Entity, position: Position) -> Option<Entity> {
//        None
//    }
//
//    pub fn remove_all_items(&mut self, position: Position) -> Option<ItemStack> {
//        None
//    }
//}

// Map Manipulation / General
impl<'w, 's> MapManager<'w, 's> {
    pub fn get_current_world_position(&self) -> WorldPosition { self.map_manager.current_map.0 }

    pub fn set_visibility(&mut self, visibility_map: VisibilityMap) {
        for position in visibility_map.iter() {
            let Some(map) = self.get_map(position.get_world_position()) else { return; };
            map.explored_tiles.insert(position.gridpoint());
        }
        self.map_manager.visible_tiles = visibility_map;
    }
}

// Internal MapManager Functions
impl<'w, 's> MapManager<'w, 's> {
    fn get_map(&mut self, world_position: WorldPosition) -> Option<&mut Map> {
        self.load_map(world_position);
        if self.map_manager.current_map.0 == world_position {
            Some(&mut self.map_manager.current_map.1)
        } else {
            self.map_manager.loaded_maps.get_mut(&world_position)
        }
    }

    fn load_map(&mut self, world_position: WorldPosition) {
        if self.is_map_loaded(world_position) ||
            self.deserialize_map(world_position) ||
            self.create_map(world_position)
        {
        } else {
            // If this fails something is terribly wrong!!!
            panic!("Error loading map at {:?}", world_position.xyz());
        }
    }

    fn is_map_loaded(&mut self, world_position: WorldPosition) -> bool {
        self.map_manager.loaded_maps.contains_key(&world_position) ||
            self.map_manager.current_map.0 == world_position
    }

    fn deserialize_map(&mut self, _world_position: WorldPosition) -> bool {
        // TODO: Deserialize maps!
        false
    }

    fn create_map(&mut self, world_position: WorldPosition) -> bool {
        let map = Self::internal_create_map(&mut self.commands, &mut self.game_context, world_position);
        info!("Generated map at {:?}", world_position.xyz());
        self.add_to_loaded_maps(world_position, map);

        true
    }

    fn add_to_loaded_maps(&mut self, world_position: WorldPosition, map: Map) {
        self.map_manager.loaded_maps.insert(world_position, map);
    }

    fn set_current_map(&mut self, world_position: WorldPosition) {
        // Check map is not current already.
        if self.map_manager.current_map.0 != world_position {
            // Verify map is loaded.
            self.load_map(world_position);

            // Get the new position/map from loaded_maps.
            let Some(map) = self.map_manager.loaded_maps.remove(&world_position) else {return;};

            // Set the new current map
            let (pos, map) = std::mem::replace(&mut self.map_manager.current_map, (world_position, map));
            // Retain the old current map in loaded_maps.
            self.add_to_loaded_maps(pos, map);
        }
    }
}

// "Static" functions
impl<'w, 's> MapManager<'w, 's> {
    fn internal_create_tilemaps(commands: &mut Commands, tilesets: &Tilesets) -> (Entity, Entity) {
        let map_size = UVec2::new(GRID_WIDTH, GRID_HEIGHT);

        let tileset = tilesets.get_by_id(&TILESET_TERRAIN_ID).expect("Cannot find TILESET_TERRAIN_ID.");
        let terrain_layer_entity = commands.spawn(Name::new("TERRAIN_LAYER".to_string())).id();

        create_tilemap_on_entity(
            commands,
            terrain_layer_entity,
            map_size,
            MapLayer::Terrain,
            tileset,
            1.0,
        );

        let tileset = tilesets.get_by_id(&TILESET_FEATURES_ID).expect("Cannot find TILESET_FEATURES_ID.");
        let features_layer_entity = commands.spawn(Name::new("FEATURES_LAYER".to_string())).id();
        create_tilemap_on_entity(
            commands,
            features_layer_entity,
            map_size,
            MapLayer::Features,
            tileset,
            1.0,
        );

        (terrain_layer_entity, features_layer_entity)
    }

    fn internal_create_map(
        commands: &mut Commands,
        game_context: &mut ResMut<GameContext>,
        world_position: WorldPosition,
    ) -> Map {
        // Create a Random for the map to be generated from and then use as it's own.
        let map_seed =
            game_context.random.prht.get(world_position.x(), world_position.y(), world_position.z());
        let random = Random::new(map_seed);

        // Create the entity to hold the map.
        let map_entity = commands.spawn_empty().id();

        // Create the map.
        let map = Self::generate_map(world_position, random, MapPassThroughData { map_entity });

        // Build the map entity.
        commands.entity(map_entity).insert((
            Name::new(format!(
                "MAP ({}, {}, {})",
                world_position.x(),
                world_position.y(),
                world_position.z()
            )),
            // map,
            SpatialBundle::default(),
        ));

        // This map is currently loaded, add it to loaded_maps
        map
    }

    fn generate_map(world_position: WorldPosition, random: Random, user_data: MapPassThroughData) -> Map {
        let mut map = Map::from(
            MapGenerator::new(
                world_position,
                random,
                SetBuilder::new().set_value(1),
                user_data,
            )
            .generate(),
        );

        let feature_map = MapGenerator::new(
            world_position,
            Random::from_entropy(), // TODO: need to generate a new random differently
            ScatterBuilder::new().with_value_array::<2>([1, 2]),
            0,
        )
        .with(
            CellularAutomataBuilder::new()
                .with_alive_value(2)
                .with_dead_value(1)
                .with_iterations(10)
                .with_shape(Circle::new(
                    Position::new(
                        world_position,
                        LocalPosition::new(GRID_WIDTH / 2, GRID_HEIGHT / 2, MapLayer::Features as u32),
                    ),
                    5u32,
                )),
        )
        .with(
            SetBuilder::new().set_value(0).with_shape(GridRectangle::new(
                Position::new(
                    world_position,
                    LocalPosition::new(
                        GRID_WIDTH / 2 - 4,
                        GRID_HEIGHT / 2 - 4,
                        MapLayer::Features as u32,
                    ),
                ),
                Position::new(
                    world_position,
                    LocalPosition::new(
                        GRID_WIDTH / 2 + 4,
                        GRID_HEIGHT / 2 + 4,
                        MapLayer::Features as u32,
                    ),
                ),
            )),
        )
        .generate();

        let mut position = Position::new(
            world_position,
            LocalPosition::new(0, 0, MapLayer::Features as u32),
        );
        for y in 0..GRID_HEIGHT {
            position.add_y(1);
            for x in 0..GRID_WIDTH {
                position.add_x(1);

                match feature_map.output_grid.get_unchecked((x, y)) {
                    0 => continue,
                    1 => continue,
                    2 => {
                        // Create Wall Feature at Position
                        map.terrain.set_unchecked(position.gridpoint(), TILE_TERRAIN_MISSING_ID);
                    },
                    _ => continue,
                }
            }
        }

        map
    }
}

// MapManager Systems
pub fn startup_map_manager(
    tilesets: Tilesets,
    mut commands: Commands,
    state: Res<CurrentAppState>,
    mut game_context: ResMut<GameContext>,
) {
    // TODO: Deserialize map
    let world_position = WorldPosition::new(0, 0, 0);
    let map = MapManager::internal_create_map(&mut commands, &mut game_context, world_position);
    let (terrain_layer, features_layer) = MapManager::internal_create_tilemaps(&mut commands, &tilesets);
    commands.insert_resource(MapManagerResource::new(
        world_position,
        map,
        terrain_layer,
        features_layer,
    ));

    // AppState::Loading(LoadingState::InitGame) -> AppState::Loading(LoadingState::WorldGen)
    switch_app_state!(AppState::Loading(LoadingState::WorldGen))
}

pub fn set_current_map_to_current_player(
    mut map_manager: MapManager,
    player_entity: Res<PlayerEntity>,
    q_positions: Query<&Position>,
) {
    if let Ok(position) = q_positions.get(player_entity.current()) {
        if map_manager.map_manager.current_map.0 != position.get_world_position() {
            info!(
                "Switching map to: WorldPosition:{}",
                position.get_world_position()
            );
            map_manager.set_current_map(position.get_world_position());
            map_manager.map_manager.current_map.1.update_all = true;
        }
    }
}

pub fn update_tilemaps(
    mut map_manager: MapManager,
    q_storage: Query<&TileStorage>,
    mut q_tiles: Query<(&mut TileTextureIndex, &mut TileVisible, &mut TileColor)>,
    // TODO: Component for holding image index on features???
    mut q_visibility: ParamSet<(
        Query<&mut Visibility>,
        Query<(&mut Visibility, &Position), With<Mob>>,
    )>,
) {
    // Get storages
    let Ok(terrain_storage) = q_storage.get(map_manager.map_manager.terrain_layer) else {
        error!("No terrain storage!");
        return;
    };

    let Ok(feature_storage) = q_storage.get(map_manager.map_manager.features_layer) else {
        error!("No feature storage!");
        return;
    };

    let mut check_next = HashSet::new();
    let map = &mut map_manager.map_manager.current_map.1;

    if map.update_all {
        for y in 0..map.size.height() {
            for x in 0..map.size.width() {
                let tile_pos = TilePos::new(x, y);

                // Update Terrain
                let Some(entity) = terrain_storage.get(&tile_pos) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Ok((mut tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let index = *map.terrain.get_unchecked(UVec2::new(x, y)) as u32;
                tile_texture_index.0 = index;

                // Update Features
                let Some(entity) = feature_storage.get(&tile_pos) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Ok((mut _tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                    check_next.insert(UVec2::new(x, y));
                    continue;
                };
                let Some(list) = map.features.get_unchecked(UVec2::new(x, y)) else {
                    continue;
                };
                let Some(_feature_entity) = list.first() else {
                    continue;
                };
                // Get feature TileId
                // tile_texture_index.0 = feature_tile_id
            }
        }

        map.update_tiles = check_next;
        map.update_all = false;
    } else {
        let mut points = std::mem::take(&mut map.update_tiles);
        for point in points.drain() {
            let tile_pos = TilePos::new(point.x, point.y);

            // Update Terrain
            let Some(entity) = terrain_storage.get(&tile_pos) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Ok((mut tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let index = *map.terrain.get_unchecked(UVec2::new(point.x, point.y)) as u32;
            tile_texture_index.0 = index;

            // Update Features
            let Some(entity) = feature_storage.get(&tile_pos) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Ok((mut _tile_texture_index, ..)) = q_tiles.get_mut(entity) else {
                check_next.insert(UVec2::new(point.x, point.y));
                continue;
            };
            let Some(list) = map.features.get_unchecked(UVec2::new(point.x, point.y)) else {
                continue;
            };
            let Some(_feature_entity) = list.first() else {
                continue;
            };
            // Get feature TileId
            // tile_texture_index.0 = feature_tile_id
        }
        map.update_tiles = check_next;
    }

    let mut position = Position::new(
        map_manager.map_manager.current_map.0,
        LocalPosition::new(0, 0, MapLayer::Terrain as u32), // MapLayer is ignored
    );

    let visible_tiles = map_manager.map_manager.visible_tiles.get_all();
    // refresh mutable reference for borrow checker...
    let (current_world_position, map) = &mut map_manager.map_manager.current_map;

    for y in 0..map.size.height() {
        position.set_y(y);
        for x in 0..map.size.width() {
            position.set_x(x);

            let tile_pos = TilePos::new(x, y);
            let is_explored = map.explored_tiles.contains(&UVec2::new(x, y));

            // Terrain
            if let Some(entity) = terrain_storage.get(&tile_pos) {
                if let Ok(mut visibility) = q_visibility.p0().get_mut(entity) {
                    visibility.is_visible = is_explored;
                }

                if let Ok((_index, mut tile_visibility, mut tile_color)) = q_tiles.get_mut(entity) {
                    tile_visibility.0 = is_explored;
                    tile_color.0.set_a(0.15);
                    if visible_tiles.contains(&position) {
                        tile_color.0.set_a(1.0);
                    }
                }
            }

            // Features
            if let Some(entity) = feature_storage.get(&tile_pos) {
                if let Ok(mut visibility) = q_visibility.p0().get_mut(entity) {
                    visibility.is_visible = map.explored_tiles.contains(&UVec2::new(x, y));
                }
            }
            // tiles too
        }
    }

    // Actors
    for (mut visibility, position) in q_visibility.p1().iter_mut() {
        if position.get_world_position() == *current_world_position {
            visibility.is_visible = visible_tiles.contains(position);
        } else {
            visibility.is_visible = false;
        }
    }
}

// Implement FovProvider
impl<'w, 's> FovProvider for MapManager<'w, 's> {
    fn is_opaque(
        &mut self,
        position: Position,
        vision_type: u8,
        q_blocks_vision: &Query<&BlocksVision>,
    ) -> bool {
        if let Some(actors) = self.get_actors(position) {
            for &entity in actors {
                if let Ok(blocks_vision) = q_blocks_vision.get(entity) {
                    if blocks_vision.is_blocked(vision_type) {
                        return true;
                    }
                }
            }
        }

        if let Some(features) = self.get_features(position) {
            for &entity in features {
                if let Ok(blocks_vision) = q_blocks_vision.get(entity) {
                    if blocks_vision.is_blocked(vision_type) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// Implement PathProvider
impl<'w, 's> PathProvider for MapManager<'w, 's> {
    fn cost(&mut self, position: Position, _movement_type: u8) -> u32 {
        1
    }

    fn is_walkable(
        &mut self,
        position: Position,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        let Some(map) = self.get_map(position.get_world_position()) else { return false; };

        map.can_place_actor(
            position.get_local_position(),
            movement_type,
            q_blocks_movement,
        )
    }
}
