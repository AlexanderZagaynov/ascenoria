use super::super::types::{CombatLog, CombatOutcome, Combatant};
use super::actions::{apply_specials, take_action};
use super::visibility::update_visibility;

/// Run a simple combat until one side is eliminated or the turn cap is reached.
pub fn simulate_combat(
    mut attackers: Vec<Combatant>,
    mut defenders: Vec<Combatant>,
    positions_attackers: Vec<(i32, i32)>,
    positions_defenders: Vec<(i32, i32)>,
    max_rounds: usize,
) -> (CombatOutcome, CombatLog) {
    let mut log = CombatLog::default();
    let mut attacker_visible = vec![true; attackers.len()];
    let mut defender_visible = vec![true; defenders.len()];

    for _ in 0..max_rounds {
        if defenders.iter().all(|c| !c.alive()) {
            return (CombatOutcome::AttackerVictory, log);
        }
        if attackers.iter().all(|c| !c.alive()) {
            return (CombatOutcome::DefenderVictory, log);
        }

        update_visibility(
            &attackers,
            &defenders,
            &positions_attackers,
            &positions_defenders,
            &mut attacker_visible,
            &mut defender_visible,
        );

        take_round(
            &mut attackers,
            &mut defenders,
            &mut log,
            &attacker_visible,
            &defender_visible,
        );
        if defenders.iter().all(|c| !c.alive()) {
            return (CombatOutcome::AttackerVictory, log);
        }
        if attackers.iter().all(|c| !c.alive()) {
            return (CombatOutcome::DefenderVictory, log);
        }
    }

    (CombatOutcome::Draw, log)
}

fn take_round(
    attackers: &mut [Combatant],
    defenders: &mut [Combatant],
    log: &mut CombatLog,
    attacker_visible: &[bool],
    defender_visible: &[bool],
) {
    // Initiative ordering: higher acts first.
    let mut acting: Vec<(bool, usize, i32)> = attackers
        .iter()
        .enumerate()
        .filter(|(_, c)| c.alive())
        .map(|(i, c)| (true, i, c.initiative))
        .chain(
            defenders
                .iter()
                .enumerate()
                .filter(|(_, c)| c.alive())
                .map(|(i, c)| (false, i, c.initiative)),
        )
        .map(|(is_attacker, idx, init)| (is_attacker, idx, init))
        .collect();

    acting.sort_by(|a, b| b.2.cmp(&a.2));

    for (is_attacker, idx, _) in acting {
        if is_attacker {
            if attacker_visible.get(idx) != Some(&true) {
                continue;
            }
            if let Some((actor, target, shield_damage, note)) =
                take_action(attackers, defenders, defender_visible, idx)
            {
                log.record(&actor, target, actor.attack, shield_damage, note);
            }
        } else {
            if defender_visible.get(idx) != Some(&true) {
                continue;
            }
            if let Some((actor, target, shield_damage, note)) =
                take_action(defenders, attackers, attacker_visible, idx)
            {
                log.record(&actor, target, actor.attack, shield_damage, note);
            }
            if let Some(actor) = defenders.get_mut(idx) {
                apply_specials(actor);
            }
        }
        if is_attacker {
            if let Some(actor) = attackers.get_mut(idx) {
                apply_specials(actor);
            }
        }
    }
}
