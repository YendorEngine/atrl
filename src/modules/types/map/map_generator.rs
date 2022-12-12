use crate::prelude::*;

pub struct MapGenerator<T> {
    map_gen_data: MapGenData<T>,
    builders: Vec<Box<dyn MapArchitect<T>>>,
}

impl<T> MapGenerator<T> {
    pub fn new(
        world_position: WorldPosition,
        random: Random,
        starter: Box<dyn MapArchitect<T>>,
        user_data: T,
    ) -> Self {
        Self {
            builders: vec![starter],
            map_gen_data: MapGenData::new(world_position, random, user_data),
        }
    }

    pub fn with(mut self, builder: Box<dyn MapArchitect<T>>) -> Self {
        self.builders.push(builder);
        self
    }

    pub fn generate(mut self) -> MapGenData<T> {
        info!("Generating map with {} builders", self.builders.len());
        for builder in self.builders.iter_mut() {
            builder.generate(&mut self.map_gen_data);
        }
        self.map_gen_data
    }
}
