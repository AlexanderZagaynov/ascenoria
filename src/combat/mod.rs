mod simulation;
mod types;

#[cfg(test)]
mod tests;

pub use simulation::simulate_combat;
pub use types::{CombatLog, CombatLogEntry, CombatOutcome, Combatant, SpecialModule};
