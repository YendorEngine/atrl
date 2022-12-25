pub mod bundles {
    mod map {
        // LayerTagged
        mod terrain_map;
        pub use terrain_map::*;
        mod feature_map;
        pub use feature_map::*;
        mod item_map;
        pub use item_map::*;
        mod actor_map;
        pub use actor_map::*;
        mod ui_map;
        pub use ui_map::*;
    }
    pub use map::*;
    mod feature;
    pub use feature::*;
    mod item;
    pub use item::*;
    mod actor;
    pub use actor::*;
    mod ui;
    pub use ui::*;

    mod position;
    pub use position::*;
}

mod chunk_position;
pub use chunk_position::*;
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
    app.register_type::<ChunkPosition>();
    register_tags(app);
}
