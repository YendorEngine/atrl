use crate::prelude::*;

// This needs to impl FromWorld not derive reflect
#[derive(Debug)]
pub struct Map {
    // Map Definitions
    pub size: UVec2,
    pub entity: Entity,
    pub random: Random,
    pub world_position: WorldPosition,

    // Update Flags
    pub update_all: bool,
    pub update_tiles: HashSet<UVec2>,
    pub explored_tiles: HashSet<UVec2>,

    // Object containers
    pub terrain: Grid<usize>,
    pub features: Grid<Option<Vec<Entity>>>,
    // pub items: Grid<Option<ItemStack>>,
    pub actors: Grid<Option<Vec<Entity>>>,
}

// Constructor: See MapPassThroughData

//// Perform terrain functions on this map
// impl Map {
//
//}

// Perform actor functions on this map
impl Map {
    /// Do not use this function!!!
    /// Use MapManager::can_place_actor instead!!!
    pub fn can_place_actor(
        &self,
        position: LocalPosition,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        let Some(index) = position.grid_index(self.size) else { return false; };

        !self.is_blocked(index, movement_type, q_blocks_movement)
    }

    /// Do not use this function!!!
    /// Use MapManager::add_actor instead!!!
    pub fn add_actor(
        &mut self,
        actor: Entity,
        position: LocalPosition,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        let Some(index) = position.grid_index(self.size) else { return false; };

        if self.can_place_actor(position, movement_type, q_blocks_movement) {
            if self.actors[index].is_none() {
                self.actors[index] = Some(Vec::new());
            }

            // Was just created above if it didn't exist.
            let actors = self.actors[index].as_mut().unwrap();

            actors.push(actor);

            true
        } else {
            false
        }
    }

    /// Do not use this function!!!
    /// Use MapManager::remove_actor instead!!!
    pub fn remove_actor(&mut self, actor: Entity, position: LocalPosition) {
        let Some(index) = position.grid_index(self.size) else { return; };

        if let Some(actors) = self.actors[index].as_mut() {
            actors.retain(|&x| x != actor);

            if actors.is_empty() {
                self.actors[index] = None;
            }
        }
    }

    /// Do not use this function!!!
    /// Use MapManager::get_actors instead!!!
    pub fn get_actors(&self, position: LocalPosition) -> Option<&Vec<Entity>> {
        let Some(index) = position.grid_index(self.size) else { return None; };
        self.actors[index].as_ref()
    }
}

// Perform feature functions on this map
impl Map {
    /// Do not use this function!!!
    /// Use MapManager::add_feature instead!!!
    pub fn add_feature(&mut self, feature: Entity, position: LocalPosition) -> bool {
        let Some(index) = position.grid_index(self.size) else { return false; };

        if self.features[index].is_none() {
            self.features[index] = Some(Vec::new());
        }

        // Was just created above if it didn't exist.
        let features = self.features[index].as_mut().unwrap();

        features.push(feature);

        true
    }

    /// Do not use this function!!!
    /// Use MapManager::remove_feature instead!!!
    pub fn remove_feature(&mut self, feature: Entity, position: LocalPosition) {
        let Some(index) = position.grid_index(self.size) else { return; };

        if let Some(features) = self.features[index].as_mut() {
            features.retain(|&x| x != feature);

            if features.is_empty() {
                self.features[index] = None;
            }
        }
    }

    /// Do not use this function!!!
    /// Use MapManager::get_features instead!!!
    pub fn get_features(&self, position: LocalPosition) -> Option<&Vec<Entity>> {
        let Some(index) = position.grid_index(self.size) else { return None; };

        self.features[index].as_ref()
    }
}

//// Perform item functions on this map
// impl Map {
//
//}

// Blocked
impl Map {
    fn is_blocked(
        &self,
        index: usize,
        movement_type: u8,
        q_blocks_movement: &Query<&BlocksMovement>,
    ) -> bool {
        // if there are actors
        if let Some(actors) = self.actors[index].as_ref() {
            // for each actor
            for &entity in actors {
                // get BlocksMovement component
                if let Ok(blocks_movement) = q_blocks_movement.get(entity) {
                    // check if we are blocked
                    if blocks_movement.is_blocked(movement_type) {
                        // if we are blocked return true
                        return true;
                    }
                }
            }
        }

        // if there are features
        if let Some(features) = self.actors[index].as_ref() {
            // for each feature
            for &entity in features {
                // get BlocksMovement component
                if let Ok(blocks_movement) = q_blocks_movement.get(entity) {
                    // check if we are blocked
                    if blocks_movement.is_blocked(movement_type) {
                        // if we are blocked return true
                        return true;
                    }
                }
            }
        }

        // We are not blocked!
        false
    }
}