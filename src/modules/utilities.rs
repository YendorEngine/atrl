pub mod error {
    mod atrl_error;
    pub use atrl_error::*;
}

pub mod file {
    mod read;
    pub use read::*;
    mod write;
    pub use write::*;
    mod find;
    pub use find::*;
}

// No need to pub use macros files. they are always exported at the `crate::*` level. Use
// src/prelude/macros.rs to list each macro individually for inclusion in the prelude.
pub mod macros {
    pub mod embed;
    pub mod generic_macros;
    pub mod primative;
    pub mod switch_app_state;
}

pub mod query_extensions {
    mod query_extensions;
    pub use query_extensions::*;
}

pub mod range {
    mod range;
    pub use range::*;
}
