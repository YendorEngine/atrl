use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TargetVisualizerStyle {
    Cursor,
    Select,
    Target,
}

impl From<TargetVisualizerStyle> for usize {
    fn from(value: TargetVisualizerStyle) -> Self {
        match value {
            TargetVisualizerStyle::Cursor => TILE_UI_CURSOR_CURSOR_ID,
            TargetVisualizerStyle::Select => TILE_UI_CURSOR_SELECT_ID,
            TargetVisualizerStyle::Target => TILE_UI_CURSOR_TARGET_ID,
        }
    }
}

#[derive(Component, Clone)]
pub struct TargetVisualizer {
    color: Color,
    end: Option<Position>,
    start: Option<Position>,
    style: TargetVisualizerStyle,
    entity_list: Vec<(Position, Entity)>,
}

impl Default for TargetVisualizer {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            end: Default::default(),
            start: Default::default(),
            entity_list: Default::default(),
            style: TargetVisualizerStyle::Cursor,
        }
    }
}

impl TargetVisualizer {
    pub const fn new(style: TargetVisualizerStyle, color: Color) -> Self {
        Self {
            color,
            style,
            entity_list: Vec::new(),
            start: None,
            end: None,
        }
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        map_manager: &MapManager,
        tilesets: &Tilesets,
        mut start: Position,
        mut end: Position,
    ) {
        start.set_layer(MapLayer::UI as u32);
        end.set_layer(MapLayer::UI as u32);
        self.start = Some(start);
        self.end = Some(end);

        // TODO: reuse entities updating position...
        let Some(tileset) = tilesets.get_by_id(&TILESET_UI_ID) else {
            error!("Couldn't find tilemap_id: {:?}. Refusing to draw TargetVisualizer.", TILESET_UI_ID);
            return;
        };

        // Wipe old visualizer
        self.clear(commands);

        let line = Line::new(start, end);
        for position in line.iter() {
            if position.get_world_position() == map_manager.get_current_world_position() &&
                map_manager.is_visible(position)
            {
                self.entity_list.push((
                    position,
                    commands
                        .spawn(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                color: self.color,
                                index: self.style.into(),
                                custom_size: Some(Vec2::ONE),
                                ..Default::default()
                            },
                            texture_atlas: tileset.atlas().clone(),
                            transform: Transform::from_translation(position.translation()),
                            ..default()
                        })
                        .id(),
                ));
            }
        }
    }

    pub fn clear(&mut self, commands: &mut Commands) {
        self.start = None;
        self.end = None;

        for (_, entity) in self.entity_list.drain(..) {
            commands.entity(entity).despawn_recursive();
        }
    }

    pub const fn get(&self) -> Option<(Position, Position)> {
        let Some(start) = self.start else {return None;};
        let Some(end) = self.end else {return None;};
        Some((start, end))
    }

    pub fn set_color(&mut self, color: Color) { self.color = color; }

    pub fn set_style(&mut self, style: TargetVisualizerStyle) { self.style = style; }
}
