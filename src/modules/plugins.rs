pub mod camera {
    mod camera_plugin;
    pub use camera_plugin::*;
}

use crate::prelude::{
    *,
    systems::systems_plugin::*,
};
// TODO: How much of this can we turn into loading systems to hide behind a Splash Screen / Loading Screen?
pub fn load_plugins(app: &mut App) {
    app.add_plugin(SystemsPlugin);
    //systems
    //raws
    //camera
    //map_rendering
    //common
    //saveload
    //spawners
    //ui
}
