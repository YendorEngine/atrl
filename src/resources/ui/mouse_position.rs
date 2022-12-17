use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct MousePosition {
    screen_coords: Vec2,
    position: ChunkPosition,
}

impl std::fmt::Display for MousePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MousePosition: {}", self.position)
    }
}

impl FromWorld for MousePosition {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<MapManager> = SystemState::from_world(world);
        let world_position = system_state.get_mut(world).get_current_world_position();

        Self::new(ChunkPosition::from(world_position))
    }
}

impl MousePosition {
    #[inline(always)]
    pub const fn new(position: ChunkPosition) -> Self {
        Self {
            position,
            screen_coords: Vec2::ZERO,
        }
    }

    #[inline(always)]
    pub const fn get_mouse_position(&self) -> ChunkPosition { self.position }

    #[inline(always)]
    pub const fn get_screen_coords(&self) -> Vec2 { self.screen_coords }

    #[inline]
    pub fn set_mouse_position(&mut self, position: ChunkPosition) { self.position = position; }

    #[inline]
    pub fn set_screen_coords(&mut self, screen_coords: Vec2) { self.screen_coords = screen_coords; }
}
