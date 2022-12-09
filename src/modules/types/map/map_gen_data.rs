use crate::prelude::*;

pub struct MapGenData<T> {
    pub user_data: T,

    pub size: UVec2,
    pub world_position: WorldPosition,
    pub random: Random,
    pub exit_positions: Vec<UVec2>,

    pub output_grid: Grid<u32>,
    pub rooms: Vec<Rectangle>,
}

impl<T> MapGenData<T> {
    pub(crate) fn new(world_position: WorldPosition, random: Random, user_data: T) -> Self {
        let size = UVec2::new(GRID_WIDTH, GRID_HEIGHT);

        Self {
            user_data,
            world_position,
            size,
            random,
            exit_positions: Vec::new(),
            output_grid: Grid::new_default(size),
            rooms: Vec::new(),
        }
    }
}
