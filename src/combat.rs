/// Minimal turn-based tactical combat loop (MVP).
///
/// This is a placeholder simulation used for testing turn flow and victory
/// resolution. It does not model spatial positioning or advanced mechanics.
#[derive(Debug, Clone, PartialEq)]
pub struct Combatant {
    pub id: String,
    pub hp: i32,
    pub shield: i32,
    pub attack: i32,
    pub initiative: i32,
    pub range: i32,
    pub scanner_range: i32,
    pub specials: Vec<SpecialModule>,
}

impl Combatant {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

/// Result of a combat engagement.
#[derive(Debug, Clone, PartialEq)]
pub enum CombatOutcome {
    AttackerVictory,
    DefenderVictory,
    Draw,
}

/// Placeholder special module effects.
#[derive(Debug, Clone, PartialEq)]
pub enum SpecialModule {
    RepairSmall,
    ShieldBoost,
}

/// Single turn log entry.
#[derive(Debug, Clone, PartialEq)]
pub struct CombatLogEntry {
    pub attacker: String,
    pub target: String,
    pub damage: i32,
    pub shield_damage: i32,
    pub target_hp_after: i32,
    pub target_shield_after: i32,
    pub note: String,
}

/// Full combat log for a skirmish.
#[derive(Debug, Default)]
pub struct CombatLog {
    pub entries: Vec<CombatLogEntry>,
}

impl CombatLog {
    fn record(
        &mut self,
        attacker: &Combatant,
        target: &Combatant,
        damage: i32,
        shield_damage: i32,
        note: String,
    ) {
        self.entries.push(CombatLogEntry {
            attacker: attacker.id.clone(),
            target: target.id.clone(),
            damage,
            shield_damage,
            target_hp_after: target.hp,
            target_shield_after: target.shield,
            note,
        });
    }
}

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

fn take_action<'a>(
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

fn apply_specials(actor: &mut Combatant) {
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

fn update_visibility(
    attackers: &[Combatant],
    defenders: &[Combatant],
    positions_attackers: &[(i32, i32)],
    positions_defenders: &[(i32, i32)],
    attacker_visible: &mut [bool],
    defender_visible: &mut [bool],
) {
    for (i, vis) in attacker_visible.iter_mut().enumerate() {
        let viewer_pos = positions_attackers.get(i).copied().unwrap_or((0, 0));
        let scanner = attackers.get(i).map(|c| c.scanner_range).unwrap_or(0);
        *vis = can_detect(scanner, viewer_pos, defenders, positions_defenders);
    }

    for (i, vis) in defender_visible.iter_mut().enumerate() {
        let viewer_pos = positions_defenders.get(i).copied().unwrap_or((0, 0));
        let scanner = defenders.get(i).map(|c| c.scanner_range).unwrap_or(0);
        *vis = can_detect(scanner, viewer_pos, attackers, positions_attackers);
    }
}

fn can_detect(
    scanner_range: i32,
    viewer_pos: (i32, i32),
    targets: &[Combatant],
    target_positions: &[(i32, i32)],
) -> bool {
    for (idx, target) in targets.iter().enumerate() {
        if !target.alive() {
            continue;
        }
        let pos = target_positions.get(idx).copied().unwrap_or((0, 0));
        let dist_sq = distance_sq(viewer_pos, pos);
        if dist_sq <= (scanner_range as i64).pow(2) {
            return true;
        }
    }
    false
}

fn distance_sq(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
}

#[cfg(test)]
mod tests {
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
}
