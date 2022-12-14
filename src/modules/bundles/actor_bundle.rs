use crate::{modules::components::*, prelude::*};

#[derive(Bundle)]
pub struct ActorBundle {
    pub mob: Mob,
    pub name: Name,
    pub health: Health,
    pub ai: AIComponent,
    pub position: PositionComponent,

    pub fov: FieldOfView,
    pub vision_component: Vision,
    pub movement_component: Movement,

    pub target_visualizer: TargetVisualizer,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
