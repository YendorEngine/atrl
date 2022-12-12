use crate::prelude::*;

pub trait MapArchitect<T, const DIM: UVec2> {
    fn generate(&mut self, data: &mut MapGenData<T, DIM>);
}
