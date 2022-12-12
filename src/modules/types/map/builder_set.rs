use std::marker::PhantomData;

use crate::prelude::*;

pub struct SetBuilder<T, const DIM: UVec2> {
    value: u32,
    phantom: PhantomData<T>,
    shapes: Vec<Box<dyn Shape>>,
}

impl<T, const DIM: UVec2> SetBuilder<T, DIM> {
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

    fn apply_shape(&self, shape: Box<dyn Shape>, data: &mut MapGenData<T, DIM>) {
        for position in shape.boxed_iter() {
            if data.world_position == position.get_world_position() {
                data.output_grid.set_unchecked(position.gridpoint(), self.value);
            }
        }
    }
}
impl<T, const DIM: UVec2> MapArchitect<T, DIM> for SetBuilder<T, DIM> {
    fn generate(&mut self, data: &mut MapGenData<T, DIM>) {
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
                Box::new(GridRectangle::new(
                    Position::new_grid_min(data.world_position),
                    Position::new_grid_max(data.world_position),
                )),
                data,
            );
        }
    }
}
