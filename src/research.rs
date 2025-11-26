use crate::data::GameData;

/// Active research item tracking progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveResearch {
    pub id: String,
    pub remaining: i32,
    pub spent: i32,
}

impl ActiveResearch {
    pub fn new(id: impl Into<String>, cost: i32) -> Self {
        Self {
            id: id.into(),
            remaining: cost.max(0),
            spent: 0,
        }
    }
}

/// Research state: selection, active tech, completed list.
#[derive(Debug, Clone)]
pub struct ResearchState {
    pub selected: usize,
    pub per_turn: i32,
    pub active: Option<ActiveResearch>,
    pub completed: Vec<String>,
}

impl ResearchState {
    pub fn new(per_turn: i32) -> Self {
        Self {
            selected: 0,
            per_turn,
            active: None,
            completed: Vec::new(),
        }
    }

    /// Cycle selection forward.
    pub fn next(&mut self, total: usize) {
        if total == 0 {
            self.selected = 0;
        } else {
            self.selected = (self.selected + 1) % total;
        }
    }

    /// Cycle selection backward.
    pub fn prev(&mut self, total: usize) {
        if total == 0 {
            self.selected = 0;
        } else if self.selected == 0 {
            self.selected = total - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// Start research on the selected tech.
    pub fn start_selected(&mut self, data: &GameData) {
        if let Some(tech) = data.techs().get(self.selected) {
            if self.completed.contains(&tech.id) {
                return;
            }
            let prereqs_met = data
                .tech_prereqs(&tech.id)
                .iter()
                .all(|id| self.completed.contains(id));
            if prereqs_met {
                self.active = Some(ActiveResearch::new(&tech.id, tech.research_cost));
            }
        }
    }

    /// Advance research by per_turn; return completed tech id if finished.
    pub fn process_turn(&mut self) -> Option<String> {
        let Some(active) = &mut self.active else {
            return None;
        };
        let spend = self.per_turn.max(0);
        active.remaining = active.remaining.saturating_sub(spend);
        active.spent += spend;
        if active.remaining == 0 {
            let id = active.id.clone();
            self.completed.push(id.clone());
            self.active = None;
            Some(id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_game_data;
    use std::path::PathBuf;

    fn fake_state() -> ResearchState {
        ResearchState::new(2)
    }

    #[test]
    fn cycles_selection() {
        let mut state = fake_state();
        state.next(3);
        assert_eq!(state.selected, 1);
        state.prev(3);
        assert_eq!(state.selected, 0);
        state.prev(3);
        assert_eq!(state.selected, 2);
    }

    #[test]
    fn processes_active_research() {
        let (data, _) = load_game_data(PathBuf::from("assets/data")).expect("load data");
        let cost = data.techs().first().map(|t| t.research_cost).unwrap_or(1);
        let mut state = ResearchState::new(cost);
        state.start_selected(&data);
        let done = state.process_turn();
        assert!(done.is_some());
        assert!(state.active.is_none());
        assert_eq!(state.completed.len(), 1);
    }
}
