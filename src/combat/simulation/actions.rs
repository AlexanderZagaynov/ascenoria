use super::super::types::{Combatant, SpecialModule};

pub fn take_action<'a>(
    actors: &'a mut [Combatant],
    targets: &'a mut [Combatant],
    targets_visible: &[bool],
    actor_idx: usize,
) -> Option<(Combatant, &'a Combatant, i32, String)> {
    if !actors.get(actor_idx).map(|c| c.alive()).unwrap_or(false) {
        return None;
    }
    let actor = actors[actor_idx].clone();
    if let Some((_, target)) = targets
        .iter_mut()
        .enumerate()
        .find(|(i, c)| c.alive() && targets_visible.get(*i) == Some(&true))
    {
        let mut remaining_damage = actor.attack.max(0);
        let mut shield_damage = 0;
        if target.shield > 0 {
            shield_damage = remaining_damage.min(target.shield);
            target.shield -= shield_damage;
            remaining_damage -= shield_damage;
        }
        if remaining_damage > 0 {
            target.hp -= remaining_damage;
        }
        let note = if shield_damage > 0 {
            "Shields hit"
        } else {
            "Hull hit"
        }
        .to_string();
        Some((actor, target, shield_damage, note))
    } else {
        None
    }
}

pub fn apply_specials(actor: &mut Combatant) {
    for effect in actor.specials.clone() {
        match effect {
            SpecialModule::RepairSmall => {
                actor.hp += 2;
            }
            SpecialModule::ShieldBoost => {
                actor.shield += 2;
            }
        }
    }
}
