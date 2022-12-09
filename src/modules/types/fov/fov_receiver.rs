use crate::prelude::*;

pub trait FovReceiver {
    fn set_visible(&mut self, position: Position);
    fn get_visible(&self, position: Position) -> bool;
    fn get_all(&self) -> HashSet<Position>;
}
