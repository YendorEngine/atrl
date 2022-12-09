pub use arrayvec::ArrayVec;
pub use bevy::{
    app::AppExit,
    core_pipeline::clear_color::ClearColorConfig,
    ecs::{
        schedule::StateData,
        system::{SystemParam, SystemState},
    },
    math::Vec3Swizzles,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode, Viewport, WindowOrigin},
        once_cell::sync::Lazy,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::{HashMap, HashSet},
    window::{PresentMode, WindowDescriptor, WindowResizeConstraints},
};
pub use bevy_ecs_tilemap::prelude::*;
pub use bevy_tileset::prelude::*;
pub use big_brain::{actions::ActionState as BigBrainActionState, prelude::*};
pub use bitvec::prelude::*;
pub use index_list::{Index, IndexList};
pub use iyes_loopless::prelude::*;
pub use iyes_progress::prelude::*;
pub use kayak_ui::{prelude::*, widgets::*};
pub use leafwing_input_manager::{action_state::ActionState, prelude::*};
pub use noise::{NoiseFn, Perlin as PerlinNoise};
pub use num_derive::*;
pub use num_traits::*;
pub use ordered_float::OrderedFloat;
pub use parking_lot::{Mutex, MutexGuard};
pub use rand::{
    distributions::{Standard, Uniform},
    Rng as RandRng,
    prelude::*,
};
pub use rand_pcg::Pcg64;
pub use ron;
pub use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
pub use smart_default::SmartDefault;
pub use thiserror::Error;
pub use xxhash_rust::xxh3::*;

#[cfg(feature = "debug")]
mod debug {
    pub use bevy_inspector_egui::{
        bevy_egui::EguiPlugin, bevy_inspector::hierarchy::SelectedEntities, egui, prelude::*, quick::*,
        DefaultInspectorConfigPlugin,
    };
    pub use egui::*;

    pub use crate::debug::*;
}

#[cfg(feature = "debug")]
pub use debug::*;
