use crate::prelude::*;

pub struct TilemapBuilder {
    map_size: UVec2,
    map_position: UVec2,
    tile_size: Option<Vec2>,
    tileset_texture: Option<Handle<Image>>,
    z_level: f32,

    tile_ids: Vec<Vec<u32>>,
}

impl TilemapBuilder {
    pub fn new(map_size: UVec2) -> Self {
        Self {
            map_size,
            map_position: UVec2::ZERO,
            tile_size: None,
            tileset_texture: None,
            z_level: 0.0,

            tile_ids: Self::generate_tile_ids(map_size),
        }
    }

    pub fn set_map_size(&mut self, map_size: UVec2) -> &mut Self {
        self.map_size = map_size;
        self.tile_ids = Self::generate_tile_ids(map_size);
        self
    }

    pub fn set_map_position(&mut self, map_position: UVec2) -> &mut Self {
        self.map_position = map_position;
        self
    }

    pub fn set_tile_size(&mut self, tile_size: Vec2) -> &mut Self {
        self.tile_size = Some(tile_size);
        self
    }

    pub fn set_tileset_texture(&mut self, tileset_texture: &Handle<Image>) -> &mut Self {
        self.tileset_texture = Some(tileset_texture.clone());
        self
    }

    pub fn set_z_level<Z: Into<f32>>(&mut self, z_level: Z) -> &mut Self {
        self.z_level = z_level.into();
        self
    }

    pub fn set_tile_id<X: Into<usize>, Y: Into<usize>, Id: Into<u32>>(
        &mut self,
        x: X,
        y: Y,
        id: Id,
    ) -> &mut Self {
        self.tile_ids[x.into()][y.into()] = id.into();
        self
    }

    pub fn clear_tile_ids(&mut self) -> &mut Self {
        self.tile_ids = Self::generate_tile_ids(self.map_size);
        self
    }

    pub fn build<B: Bundle>(&self, commands: &mut Commands, map_bundle: B) -> Entity {
        // unwrap params
        let tile_size = self.tile_size.expect("Tile size not set");
        let tileset_texture = match &self.tileset_texture {
            None => panic!("Tileset texture not set"),
            Some(texture) => texture.clone(),
        };

        // spawn map entity
        let map_entity = commands.spawn(map_bundle).id();

        // create tile storage
        let mut tile_storage = TileStorage::empty(self.map_size.into());
        for x in 0..self.map_size.x {
            for y in 0..self.map_size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(map_entity),
                        texture_index: TileTextureIndex(self.tile_ids[x as usize][y as usize]),
                        ..Default::default()
                    })
                    .id();
                commands.entity(map_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        // setup transform
        let transform = Transform {
            translation: Vec3 {
                x: (self.map_position.x * self.map_size.x) as f32 + 0.5,
                y: (self.map_position.y * self.map_size.y) as f32 + 0.5,
                z: self.z_level,
            },
            scale: Vec3 {
                x: 1.0 / tile_size.x,
                y: 1.0 / tile_size.y,
                z: 1.0,
            },
            ..Default::default()
        };

        // build tilemap
        commands.entity(map_entity).insert(TilemapBundle {
            tile_size: tile_size.into(),
            grid_size: tile_size.into(),
            size: TilemapSize {
                x: self.map_size.x,
                y: self.map_size.y,
            },
            storage: tile_storage,
            texture: TilemapTexture::Single(tileset_texture),
            transform,
            ..Default::default()
        });

        map_entity
    }

    fn generate_tile_ids(map_size: UVec2) -> Vec<Vec<u32>> {
        let mut tile_ids = Vec::with_capacity(map_size.x as usize);

        (0..map_size.x).for_each(|_| {
            let mut column = Vec::with_capacity(map_size.y as usize);
            (0..map_size.y).for_each(|_| column.push(0));
            tile_ids.push(column);
        });

        tile_ids
    }
}
