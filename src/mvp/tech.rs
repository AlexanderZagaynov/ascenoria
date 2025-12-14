#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tech {
    Hull,
    Engine,
    Generator,
}

pub const TECH_ORDER: [Tech; 3] = [Tech::Hull, Tech::Engine, Tech::Generator];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchState {
    pub current_index: usize,
    pub progress: i32,
    pub completed: [bool; 3],
    pub cost_per_tech: i32,
}

impl Default for ResearchState {
    fn default() -> Self {
        Self {
            current_index: 0,
            progress: 0,
            completed: [false; 3],
            cost_per_tech: 10,
        }
    }
}

impl ResearchState {
    pub fn current_tech(&self) -> Option<Tech> {
        TECH_ORDER.get(self.current_index).copied()
    }

    pub fn all_researched(&self) -> bool {
        self.completed.iter().all(|x| *x)
    }

    pub fn advance_with_science(&mut self, mut science: i32) -> i32 {
        while science > 0 {
            let Some(_) = self.current_tech() else {
                return science;
            };
            if self.completed[self.current_index] {
                self.current_index += 1;
                self.progress = 0;
                continue;
            }

            let remaining = self.cost_per_tech - self.progress;
            let spend = remaining.min(science);
            self.progress += spend;
            science -= spend;
            if self.progress >= self.cost_per_tech {
                self.completed[self.current_index] = true;
                self.current_index += 1;
                self.progress = 0;
            }
        }
        science
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn victory_when_all_techs_researched() {
        let mut research = ResearchState {
            cost_per_tech: 3,
            ..Default::default()
        };

        for _ in 0..3 {
            research.advance_with_science(3);
        }

        assert!(research.all_researched());
    }
}
