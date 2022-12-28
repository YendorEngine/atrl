pub mod systems {
    pub mod functions {
        mod generate;
        pub use generate::*;
    }

    mod init {
        mod generator_config;
        pub use generator_config::*;
        mod spawn_grid;
        pub use spawn_grid::*;
    }
    pub use init::*;

    pub mod run {
        mod universe_generation;
        pub use universe_generation::*;
        mod update_tilemap;
        pub use update_tilemap::*;
    }
    pub use run::*;
}

pub mod types {
    mod generator;
    pub use generator::*;
}

mod test;
pub use test::*;
