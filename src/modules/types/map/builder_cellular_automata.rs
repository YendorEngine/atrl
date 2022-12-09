use std::marker::PhantomData;

use crate::prelude::*;

const DEFAULT_ITERATIONS: u32 = 10;
const DEAD_VALUE: u32 = 0;
const ALIVE_VALUE: u32 = 1;

pub struct CellularAutomataBuilder<T> {
    phantom: PhantomData<T>,
    shapes: Vec<BoxedShape>,

    number_of_iterations: u32,
    alive_value: u32,
    dead_value: u32,
}

impl<T> CellularAutomataBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            shapes: Vec::new(),
            number_of_iterations: DEFAULT_ITERATIONS,
            alive_value: ALIVE_VALUE,
            dead_value: DEAD_VALUE,
            phantom: PhantomData,
        })
    }

    pub fn with_shape<S: Into<BoxedShape>>(mut self, shape: S) -> Box<Self> {
        self.shapes.push(shape.into());
        Box::new(self)
    }

    pub fn with_alive_value(mut self, alive_value: u32) -> Box<Self> {
        self.alive_value = alive_value;
        Box::new(self)
    }

    pub fn with_dead_value(mut self, dead_value: u32) -> Box<Self> {
        self.dead_value = dead_value;
        Box::new(self)
    }

    pub fn with_iterations(mut self, number_of_iterations: u32) -> Box<Self> {
        self.number_of_iterations = number_of_iterations;
        Box::new(self)
    }

    fn count_neighbors(grid: &Grid<u32>, index: IVec2, alive_value: u32) -> u32 {
        let mut neighbors = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let value = match grid.get((x + index.x(), y + index.y())) {
                    Some(v) => *v,
                    None => continue,
                };
                if value == alive_value {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn apply_shape<S: Into<BoxedShape>>(
        &self,
        shape: S,
        world_position: WorldPosition,
        output_grid: &mut Grid<u32>,
    ) {
        let shape: BoxedShape = shape.into();
        for position in shape.boxed_iter() {
            if world_position == position.get_world_position() {
                let grid_point = position.gridpoint().as_ivec2();
                let neighbors = Self::count_neighbors(output_grid, grid_point, self.alive_value);
                if neighbors > 4 || neighbors == 0 {
                    output_grid.set(grid_point, self.dead_value);
                } else {
                    output_grid.set(grid_point, self.alive_value);
                }
            }
        }
    }
}

impl<T> MapArchitect<T> for CellularAutomataBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        for _ in 0..self.number_of_iterations {
            let mut new_grid = data.output_grid.clone();

            if !self.shapes.is_empty() {
                loop {
                    let shape = self.shapes.pop().unwrap();
                    self.apply_shape(shape, data.world_position, &mut new_grid);

                    if self.shapes.is_empty() {
                        break;
                    }
                }
            } else {
                self.apply_shape(
                    GridRectangle::new_grid_sized(data.world_position),
                    data.world_position,
                    &mut new_grid,
                );
            }

            data.output_grid = new_grid;
        }
    }
}
