/// Configuration for domination victory checks.
#[derive(Debug, Clone, Copy)]
pub struct DominationConfig {
    /// Fraction of systems required to win (0.0-1.0].
    pub threshold: f32,
}

impl Default for DominationConfig {
    fn default() -> Self {
        Self { threshold: 0.5 }
    }
}

/// Tracks domination victory state.
#[derive(Debug, Clone)]
pub struct VictoryState {
    pub total_systems: i32,
    pub controlled_systems: i32,
    pub domination_achieved: bool,
    pub config: DominationConfig,
    pub tech_victory: bool,
    pub ascension_victory: bool,
}

impl VictoryState {
    pub fn new(total_systems: i32, config: DominationConfig) -> Self {
        Self {
            total_systems: total_systems.max(0),
            controlled_systems: 0,
            domination_achieved: false,
            config,
            tech_victory: false,
            ascension_victory: false,
        }
    }

    /// Compute whether domination threshold is met.
    pub fn check_domination(&mut self) -> bool {
        if self.total_systems == 0 {
            return false;
        }
        let required = (self.total_systems as f32 * self.config.threshold).ceil() as i32;
        if self.controlled_systems >= required {
            self.domination_achieved = true;
        }
        self.domination_achieved
    }

    pub fn check_tech_victory(&mut self, total_techs: usize, completed: usize) -> bool {
        if total_techs == 0 {
            return false;
        }
        if completed >= total_techs {
            self.tech_victory = true;
        }
        self.tech_victory
    }

    pub fn mark_ascension_victory(&mut self) {
        self.ascension_victory = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_domination_when_threshold_met() {
        let mut state = VictoryState::new(10, DominationConfig { threshold: 0.6 });
        state.controlled_systems = 6;
        assert!(state.check_domination());
        assert!(state.domination_achieved);
    }

    #[test]
    fn no_domination_when_below_threshold() {
        let mut state = VictoryState::new(10, DominationConfig { threshold: 0.7 });
        state.controlled_systems = 6;
        assert!(!state.check_domination());
        assert!(!state.domination_achieved);
    }

    #[test]
    fn handles_zero_systems() {
        let mut state = VictoryState::new(0, DominationConfig::default());
        state.controlled_systems = 0;
        assert!(!state.check_domination());
    }
}
