use std::marker::PhantomData;

use crate::prelude::*;

pub struct SetBuilder<T> {
    value: u32,
    shapes: Vec<BoxedShape>,
    phantom: PhantomData<T>,
}

impl<T> SetBuilder<T> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            value: u32::MIN,
            shapes: Vec::new(),
            phantom: PhantomData,
        })
    }

    pub fn with_shape<S: Into<BoxedShape>>(mut self, shape: S) -> Box<Self> {
        self.shapes.push(shape.into());
        Box::new(self)
    }

    pub fn set_value(mut self, value: u32) -> Box<Self> {
        self.value = value;
        Box::new(self)
    }

    fn apply_shape<S: Into<BoxedShape>>(&self, shape: S, data: &mut MapGenData<T>) {
        let shape: BoxedShape = shape.into();
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
            self.apply_shape(GridRectangle::new_grid_sized(data.world_position), data);
        }
    }
}
