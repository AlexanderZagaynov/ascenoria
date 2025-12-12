use super::*;

fn attacker(
    id: &str,
    hp: i32,
    shield: i32,
    attack: i32,
    initiative: i32,
    range: i32,
    scanner_range: i32,
) -> Combatant {
    Combatant {
        id: id.to_string(),
        hp,
        shield,
        attack,
        initiative,
        range,
        scanner_range,
        specials: Vec::new(),
    }
}

#[test]
fn attacker_wins_when_defenders_eliminated() {
    let attackers = vec![attacker("a1", 10, 0, 5, 5, 3, 3)];
    let defenders = vec![attacker("d1", 5, 0, 1, 1, 3, 3)];

    let (outcome, log) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 5);
    assert_eq!(outcome, CombatOutcome::AttackerVictory);
    assert!(!log.entries.is_empty());
}

#[test]
fn defender_wins_when_attackers_eliminated() {
    let attackers = vec![attacker("a1", 5, 0, 1, 1, 3, 3)];
    let defenders = vec![attacker("d1", 10, 0, 5, 5, 3, 3)];

    let (outcome, _) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 5);
    assert_eq!(outcome, CombatOutcome::DefenderVictory);
}

#[test]
fn draw_when_round_limit_reached() {
    let attackers = vec![attacker("a1", 1, 0, 0, 1, 3, 3)];
    let defenders = vec![attacker("d1", 1, 0, 0, 1, 3, 3)];

    let (outcome, _) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 2);
    assert_eq!(outcome, CombatOutcome::Draw);
}

#[test]
fn honors_initiative_order() {
    let attackers = vec![attacker("a1", 5, 0, 5, 10, 3, 3)];
    let defenders = vec![attacker("d1", 5, 0, 1, 1, 3, 3)];

    let (_, log) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 1);
    assert_eq!(log.entries.first().unwrap().attacker, "a1");
}

#[test]
fn shields_absorb_before_hull() {
    let attackers = vec![attacker("a1", 10, 0, 5, 5, 3, 3)];
    let defenders = vec![attacker("d1", 5, 3, 1, 1, 3, 3)];

    let (_, log) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 1);
    let entry = &log.entries[0];
    assert_eq!(entry.shield_damage, 3);
    assert_eq!(entry.target_shield_after, 0);
    assert_eq!(entry.target_hp_after, 3);
}

#[test]
fn shield_overflow_hits_hull() {
    let attackers = vec![attacker("a1", 10, 0, 5, 5, 3, 3)];
    let defenders = vec![attacker("d1", 5, 2, 1, 1, 3, 3)];

    let (_, log) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(1, 0)], 1);
    let entry = &log.entries[0];
    assert_eq!(entry.shield_damage, 2);
    assert_eq!(entry.target_shield_after, 0);
    assert_eq!(entry.target_hp_after, 2);
}

#[test]
fn invisible_targets_are_not_attacked() {
    let attackers = vec![attacker("a1", 10, 0, 5, 5, 3, 2)];
    let defenders = vec![attacker("d1", 5, 0, 1, 1, 3, 1)];

    let (outcome, log) = simulate_combat(attackers, defenders, vec![(0, 0)], vec![(10, 0)], 2);
    assert_eq!(outcome, CombatOutcome::Draw);
    assert!(
        log.entries.is_empty(),
        "No attacks when out of scanner range"
    );
}
