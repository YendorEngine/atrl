pub mod asset_ids {
    pub mod tilesets {
        pub mod tiny_galaxy {
            mod fx;
            pub use fx::*;
            mod interface;
            pub use interface::*;
            mod items;
            pub use items::*;
            mod monsters;
            pub use monsters::*;
            mod portraits;
            pub use portraits::*;
            mod space;
            pub use space::*;
            mod world;
            pub use world::*;
        }
        pub use tiny_galaxy::*;

        pub mod white_pixel {
            mod white_pixel;
            pub use white_pixel::*;
        }
        pub use white_pixel::*;
    }
}

pub mod input {
    mod movement;
    pub use movement::*;
}

pub mod shapes {
    mod shape;
    pub use shape::*;

    mod circle;
    pub use circle::*;
    mod line;
    pub use line::*;
    mod line_iter;
    pub use line_iter::*;
    mod octant;
    pub use octant::*;
}
