pub mod systems {
    pub mod functions {
        mod generate;
        pub use generate::*;
    }

    mod init {
        mod generator_config;
        pub use generator_config::*;
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
    mod generator_config;
    pub use generator_config::*;
    mod generator_type;
    pub use generator_type::*;
    mod noise_config;
    pub use noise_config::*;
    mod spiral_config;
    pub use spiral_config::*;
}

mod test;
pub use test::*;
