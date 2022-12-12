use crate::prelude::*;

pub trait MapArchitect<T> {
    fn generate(&mut self, data: &mut MapGenData<T>);
}
