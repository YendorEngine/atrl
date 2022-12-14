use crate::prelude::*;

// We assume the attack range is sqrt(2)
pub fn in_attack_range(attacker_pos: Position, victim_pos: Position) -> bool {
    let distance = attacker_pos.distance(victim_pos);
    distance <= 1
}
