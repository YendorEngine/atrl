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
    }
}

pub mod input {
    mod movement;
    pub use movement::*;
}

mod resolution;
pub use resolution::*;
