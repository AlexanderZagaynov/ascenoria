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
    pub fn record(
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
