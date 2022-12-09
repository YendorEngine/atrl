use crate::prelude::*;

pub const SECONDS: u32 = 1000;
pub const MINUTES: u32 = SECONDS * 60;
pub const HOURS: u32 = MINUTES * 60;
pub const DAYS: u32 = HOURS * 24;

pub const TURN_TIME: u32 = DAYS; // 86_400_000;

#[derive(Default, Resource)]
pub struct TurnManager {
    turn_number: u32,
    current_time: u32,
    entities: IndexList<(u32, u32, Entity)>,
}

// Actor Turns
impl TurnManager {
    /// Add entities to the TurnManager when building the map.
    pub fn add_entity(&mut self, entity: Entity) {
        if let Some((turn_number, current_time, _entity)) = self.entities.get_last() {
            self.entities.insert_last((*turn_number, *current_time, entity));
        } else {
            self.entities.insert_last((self.turn_number, self.current_time, entity));
        }
    }

    /// Remove an entity when it dies or the map unloads.
    pub fn remove_entity(&mut self, entity: Entity) {
        let index = self.entities.first_index();
        let mut found_index = None;
        while index.is_some() {
            if let Some((_turn_number, _current_time, current_entity)) = self.entities.get(index) {
                if *current_entity == entity {
                    found_index = Some(index);
                    break;
                }
            }
        }

        if let Some(index) = found_index {
            self.entities.remove(index);
        }
    }

    /// Clear all entities
    /// possibly reset the turn_number
    pub fn clear_entities(&mut self, reset_turn_number: bool) {
        if reset_turn_number {
            self.turn_number = 0;
        }
        self.current_time = 0;
        self.entities.clear();
    }

    /// This will get the next entity who is ready for a new turn
    /// once you get the entity from this, match it to either:
    /// ```
    /// // check this first as it's more common:
    /// if let Ok((AI_QUERY_COMPONENTS)) = q_ai_components.get_mut() {[...]}
    /// ```
    /// or
    /// ```
    /// if let Ok((PLAYER_QUERY_COMPONENTS)) = q_player_components.get_mut() {[...]}
    /// ```
    pub fn start_entity_turn(&mut self) -> Option<Entity> {
        if let Some((turn_number, current_time, entity)) = self.entities.remove_first() {
            if self.turn_number != turn_number {
                info!("Starting Turn: {}", turn_number);
            }
            // we are at least to turn_number:current_time
            self.turn_number = turn_number;
            self.current_time = current_time;
            return Some(entity);
        }
        None
    }

    /// After the entity performs an action,
    /// call this to re-add the entity to the queue
    /// time_spent is the amount of time used to perform the action.
    pub fn end_entity_turn(&mut self, entity: Entity, time_spent: u32) {
        // shortcut no time spent:
        if time_spent == 0 {
            self.entities.insert_first((self.turn_number, self.current_time, entity));
            return;
        }
        info!("Turn took {}", time_spent);

        let mut next_turn = self.turn_number;
        let mut next_time = self.current_time + time_spent;

        loop {
            if next_time < TURN_TIME {
                break;
            }
            next_turn += 1;
            next_time -= TURN_TIME;
        }

        if let Some(index) = self.get_index_after_time(next_turn, next_time) {
            self.entities.insert_before(index, (next_turn, next_time, entity));
        } else {
            self.entities.insert_last((next_turn, next_time, entity));
        }
    }

    fn get_index_after_time(&self, turn_number: u32, time: u32) -> Option<Index> {
        let mut index = self.entities.first_index();
        while index.is_some() {
            if let Some((current_turn_number, current_time, _entity)) = self.entities.get(index) {
                if *current_turn_number > turn_number ||
                    (*current_turn_number == turn_number && *current_time > time)
                {
                    return Some(index);
                }
            }
            index = self.entities.next_index(index);
        }
        None
    }
}

// Time
impl TurnManager {
    /// Which Day it is
    /// starting at Day 0
    #[inline]
    pub const fn get_days(&self) -> u32 { self.current_time / DAYS }

    /// What Hour of the Day it is
    /// 0..=23
    #[inline]
    pub const fn get_hours(&self) -> u32 { (self.current_time - self.get_days() * DAYS) / HOURS }

    /// What Minute of the Hour it is
    /// 0..=59
    #[inline]
    pub const fn get_minutes(&self) -> u32 {
        (self.current_time - self.get_days() * DAYS - self.get_hours() * HOURS) / MINUTES
    }

    /// What Second of the Minute it is
    /// 0..=(SECONDS * 60) - 1
    #[inline]
    pub const fn get_seconds(&self) -> u32 {
        self.current_time - self.get_days() * DAYS - self.get_hours() * HOURS - self.get_minutes() * MINUTES
    }
}
