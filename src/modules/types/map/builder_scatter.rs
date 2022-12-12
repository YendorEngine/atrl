use std::marker::PhantomData;

use crate::prelude::*;

pub struct ScatterBuilder<T, const DIM: UVec2> {
    shapes: Vec<Box<dyn Shape>>,
    values: Vec<u32>,

    phantom: PhantomData<T>,
}

impl<T, const DIM: UVec2> ScatterBuilder<T, DIM> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            shapes: Vec::new(),
            values: Vec::new(),
            phantom: PhantomData,
        })
    }

    pub fn with_shape(mut self, shape: impl Shape + 'static) -> Box<Self> {
        self.shapes.push(Box::new(shape));
        Box::new(self)
    }

    pub fn with_value<V: Into<u32>>(mut self, value: V) -> Box<Self> {
        self.values.push(value.into());
        Box::new(self)
    }

    pub fn with_values(mut self, values: &Vec<u32>) -> Box<Self> {
        for value in values {
            self.values.push(*value);
        }

        Box::new(self)
    }

    pub fn with_value_array<const COUNT: usize>(mut self, values: [u32; COUNT]) -> Box<Self> {
        values.iter().take(COUNT).for_each(|value| {
            self.values.push(*value);
        });

        Box::new(self)
    }

    fn apply_shape(&mut self, shape: Box<dyn Shape>, data: &mut MapGenData<T, DIM>, values: &Vec<u32>) {
        // let world_position = data.world_position;
        let rng = &mut data.random.prng;
        let length = values.len() as u32 - 1;
        for position in shape.boxed_iter() {
            if data.world_position == position.get_world_position() {
                let index = rng.range(0..length) as usize;
                data.output_grid.set_unchecked(position.gridpoint(), values[index]);
            }
        }
    }
}

impl<T, const DIM: UVec2> MapArchitect<T, DIM> for ScatterBuilder<T, DIM> {
    fn generate(&mut self, data: &mut MapGenData<T, DIM>) {
        let mut values = self.values.clone();
        if values.is_empty() {
            values.push(0);
        }

        if !self.shapes.is_empty() {
            loop {
                if self.shapes.is_empty() {
                    break;
                }
                let shape = self.shapes.pop().unwrap();
                self.apply_shape(shape, data, &values);
            }
        } else {
            self.apply_shape(
                Box::new(Rectangle::new(
                    Position::new_grid_min(data.world_position),
                    Position::new_grid_max(data.world_position),
                )),
                data,
                &values,
            );
        }
    }
}
