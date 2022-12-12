use crate::prelude::*;

pub struct MapGenerator<T, const DIM: UVec2> {
    map_gen_data: MapGenData<T, DIM>,
    builders: Vec<Box<dyn MapArchitect<T, DIM>>>,
}

impl<T, const DIM: UVec2> MapGenerator<T, DIM> {
    pub fn new(
        world_position: WorldPosition,
        random: Random,
        starter: Box<dyn MapArchitect<T, DIM>>,
        user_data: T,
    ) -> Self {
        Self {
            builders: vec![starter],
            map_gen_data: MapGenData::new(world_position, random, user_data),
        }
    }

    pub fn with(mut self, builder: Box<dyn MapArchitect<T, DIM>>) -> Self {
        self.builders.push(builder);
        self
    }

    pub fn generate(mut self) -> MapGenData<T, DIM> {
        info!("Generating map with {} builders", self.builders.len());
        for builder in self.builders.iter_mut() {
            builder.generate(&mut self.map_gen_data);
        }
        self.map_gen_data
    }
}
