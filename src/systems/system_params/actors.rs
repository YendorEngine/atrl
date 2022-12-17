use crate::{components::*, prelude::*, resources::*};

#[derive(SystemParam)]
pub struct Actors<'w, 's> {
    player_entity: ResMut<'w, PlayerEntity>,
    q_actors: Query<
        'w,
        's,
        (
            Entity,
            (
                &'static mut PositionComponent,
                &'static mut DisplayComponent,
            ),
        ),
    >,
}

impl<'w, 's> Actors<'w, 's> {
    pub fn player(&self) -> Entity { self.player_entity.0 }

    pub fn position(&self, entity: Entity) -> Result<PositionComponent> {
        let (_entity, (position, ..)) = self.q_actors.get(entity)?;
        Ok(*position)
    }
}

impl<'w, 's> Actors<'w, 's> {
    pub fn set_player(&mut self, entity: Entity) { self.player_entity.0 = entity; }
}
