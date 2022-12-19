pub mod bundles {
    // LayerTagged
    mod terrain;
    pub use terrain::*;
    mod feature;
    pub use feature::*;
    mod item;
    pub use item::*;
    mod actor;
    pub use actor::*;
    mod ui;
    pub use ui::*;

    mod chunk;
    pub use chunk::*;
    mod position;
    pub use position::*;
}

mod chunk;
pub use chunk::*;
mod display;
pub use display::*;
mod local_position;
pub use local_position::*;
mod world_position;
pub use world_position::*;
mod tags;
pub use tags::*;

use crate::prelude::*;
pub(super) fn register_components(app: &mut App) {
    app.register_type::<DisplayComponent>();
    app.register_type::<WorldPositionComponent>();
    app.register_type::<LocalPositionComponent>();
    app.register_type::<ChunkComponent>();
    register_tags(app);
}