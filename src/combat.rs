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
    max_rounds: usize,
) -> (CombatOutcome, CombatLog) {
    let mut log = CombatLog::default();

    for _ in 0..max_rounds {
        if defenders.iter().all(|c| !c.alive()) {
            return (CombatOutcome::AttackerVictory, log);
        }
        if attackers.iter().all(|c| !c.alive()) {
            return (CombatOutcome::DefenderVictory, log);
        }

        take_round(&mut attackers, &mut defenders, &mut log);
        if defenders.iter().all(|c| !c.alive()) {
            return (CombatOutcome::AttackerVictory, log);
        }
        if attackers.iter().all(|c| !c.alive()) {
            return (CombatOutcome::DefenderVictory, log);
        }
    }

    (CombatOutcome::Draw, log)
}

fn take_round(attackers: &mut [Combatant], defenders: &mut [Combatant], log: &mut CombatLog) {
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
            if let Some((actor, target, shield_damage, note)) =
                take_action(attackers, defenders, idx)
            {
                log.record(&actor, target, actor.attack, shield_damage, note);
            }
        } else if let Some((actor, target, shield_damage, note)) =
            take_action(defenders, attackers, idx)
        {
            log.record(&actor, target, actor.attack, shield_damage, note);
        }
    }
}

fn take_action<'a>(
    actors: &'a mut [Combatant],
    targets: &'a mut [Combatant],
    actor_idx: usize,
) -> Option<(Combatant, &'a Combatant, i32, String)> {
    if !actors.get(actor_idx).map(|c| c.alive()).unwrap_or(false) {
        return None;
    }
    let actor = actors[actor_idx].clone();
    if let Some((_, target)) = targets.iter_mut().enumerate().find(|(_, c)| c.alive()) {
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
    ) -> Combatant {
        Combatant {
            id: id.to_string(),
            hp,
            shield,
            attack,
            initiative,
            range,
        }
    }

    #[test]
    fn attacker_wins_when_defenders_eliminated() {
        let attackers = vec![attacker("a1", 10, 0, 5, 5, 3)];
        let defenders = vec![attacker("d1", 5, 0, 1, 1, 3)];

        let (outcome, log) = simulate_combat(attackers, defenders, 5);
        assert_eq!(outcome, CombatOutcome::AttackerVictory);
        assert!(!log.entries.is_empty());
    }

    #[test]
    fn defender_wins_when_attackers_eliminated() {
        let attackers = vec![attacker("a1", 5, 0, 1, 1, 3)];
        let defenders = vec![attacker("d1", 10, 0, 5, 5, 3)];

        let (outcome, _) = simulate_combat(attackers, defenders, 5);
        assert_eq!(outcome, CombatOutcome::DefenderVictory);
    }

    #[test]
    fn draw_when_round_limit_reached() {
        let attackers = vec![attacker("a1", 1, 0, 0, 1, 3)];
        let defenders = vec![attacker("d1", 1, 0, 0, 1, 3)];

        let (outcome, _) = simulate_combat(attackers, defenders, 2);
        assert_eq!(outcome, CombatOutcome::Draw);
    }

    #[test]
    fn honors_initiative_order() {
        let attackers = vec![attacker("a1", 5, 0, 5, 10, 3)];
        let defenders = vec![attacker("d1", 5, 0, 1, 1, 3)];

        let (_, log) = simulate_combat(attackers, defenders, 1);
        assert_eq!(log.entries.first().unwrap().attacker, "a1");
    }

    #[test]
    fn shields_absorb_before_hull() {
        let attackers = vec![attacker("a1", 10, 0, 5, 5, 3)];
        let defenders = vec![attacker("d1", 5, 3, 1, 1, 3)];

        let (_, log) = simulate_combat(attackers, defenders, 1);
        let entry = &log.entries[0];
        assert_eq!(entry.shield_damage, 3);
        assert_eq!(entry.target_shield_after, 0);
        assert_eq!(entry.target_hp_after, 3);
    }

    #[test]
    fn shield_overflow_hits_hull() {
        let attackers = vec![attacker("a1", 10, 0, 5, 5, 3)];
        let defenders = vec![attacker("d1", 5, 2, 1, 1, 3)];

        let (_, log) = simulate_combat(attackers, defenders, 1);
        let entry = &log.entries[0];
        assert_eq!(entry.shield_damage, 2);
        assert_eq!(entry.target_shield_after, 0);
        assert_eq!(entry.target_hp_after, 2);
    }
}
