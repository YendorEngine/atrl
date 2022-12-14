use std::marker::PhantomData;

use super::*;
use crate::prelude::*;

pub struct SetBuilder<T> {
    value: u32,
    phantom: PhantomData<T>,
    shapes: Vec<Box<dyn Shape>>,
}

impl<T> SetBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            value: u32::MIN,
            shapes: Vec::new(),
            phantom: PhantomData,
        })
    }

    pub fn with_shape(mut self, shape: impl Shape + 'static) -> Box<Self> {
        self.shapes.push(Box::new(shape));
        Box::new(self)
    }

    pub fn set_value(mut self, value: u32) -> Box<Self> {
        self.value = value;
        Box::new(self)
    }

    fn apply_shape(&self, shape: Box<dyn Shape>, data: &mut MapGenData<T>) {
        for position in shape.boxed_iter() {
            if data.world_position == position.get_world_position() {
                data.output_grid.set_unchecked(position.gridpoint(), self.value);
            }
        }
    }
}
impl<T> MapArchitect<T> for SetBuilder<T> {
    fn generate(&mut self, data: &mut MapGenData<T>) {
        if !self.shapes.is_empty() {
            loop {
                let shape = self.shapes.pop().unwrap();
                self.apply_shape(shape, data);

                if self.shapes.is_empty() {
                    break;
                }
            }
        } else {
            self.apply_shape(
                Box::new(Rectangle::new(
                    Position::new_grid_min(data.world_position),
                    Position::new_grid_max(data.world_position),
                )),
                data,
            );
        }
    }
}
