use super::MapGenData;

pub trait MapArchitect<T> {
    fn generate(&mut self, data: &mut MapGenData<T>);
}
