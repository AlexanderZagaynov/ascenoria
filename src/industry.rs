use crate::data::{GameData, GameRegistry};

/// Category of build order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildKind {
    Surface,
    Orbital,
    Project,
}

/// Pending build order with remaining industry cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildOrder {
    pub kind: BuildKind,
    pub id: String,
    pub remaining_cost: i32,
}

/// Completed build result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedOrder {
    pub kind: BuildKind,
    pub id: String,
}

/// Planetary industry state: production plus build queue.
#[derive(Debug, Default)]
pub struct PlanetIndustry {
    pub production: i32,
    pub queue: Vec<BuildOrder>,
    pub completed: Vec<CompletedOrder>,
}

impl PlanetIndustry {
    pub fn new(production: i32) -> Self {
        Self {
            production,
            queue: Vec::new(),
            completed: Vec::new(),
        }
    }

    /// Enqueue a build order with the given cost.
    pub fn enqueue(&mut self, kind: BuildKind, id: impl Into<String>, cost: i32) {
        self.queue.push(BuildOrder {
            kind,
            id: id.into(),
            remaining_cost: cost.max(0),
        });
    }

    /// Reorder an item up in the queue.
    pub fn move_up(&mut self, index: usize) {
        if index == 0 || index >= self.queue.len() {
            return;
        }
        self.queue.swap(index, index - 1);
    }

    /// Reorder an item down in the queue.
    pub fn move_down(&mut self, index: usize) {
        if index + 1 >= self.queue.len() {
            return;
        }
        self.queue.swap(index, index + 1);
    }

    /// Process one turn of production, completing items as needed.
    pub fn process_turn(&mut self) {
        let mut remaining = self.production;
        while remaining > 0 {
            let Some(front) = self.queue.first_mut() else {
                break;
            };
            let spend = remaining.min(front.remaining_cost);
            front.remaining_cost -= spend;
            remaining -= spend;

            if front.remaining_cost == 0 {
                let done = self.queue.remove(0);
                self.completed.push(CompletedOrder {
                    kind: done.kind,
                    id: done.id,
                });
            }
        }
    }
}

/// Helper to fetch industry cost from loaded data.
pub fn industry_cost(
    data: &GameData,
    registry: &GameRegistry,
    kind: &BuildKind,
    id: &str,
) -> Option<i32> {
    match kind {
        BuildKind::Surface => registry
            .surface_item(data, id.to_string())
            .map(|item| item.industry_cost),
        BuildKind::Orbital => registry
            .orbital_item(data, id.to_string())
            .map(|item| item.industry_cost),
        BuildKind::Project => registry
            .planetary_project(data, id.to_string())
            .map(|proj| proj.industry_cost),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_game_data;
    use std::path::PathBuf;

    #[test]
    fn processes_queue_until_complete() {
        let mut industry = PlanetIndustry::new(5);
        industry.enqueue(BuildKind::Surface, "factory", 3);
        industry.enqueue(BuildKind::Orbital, "scanner", 4);

        industry.process_turn();
        assert_eq!(industry.completed.len(), 1);
        assert_eq!(industry.queue.len(), 1);
        assert_eq!(industry.queue[0].remaining_cost, 2);
    }

    #[test]
    fn reorders_queue_items() {
        let mut industry = PlanetIndustry::new(1);
        industry.enqueue(BuildKind::Surface, "a", 1);
        industry.enqueue(BuildKind::Surface, "b", 1);
        industry.move_down(0);
        assert_eq!(industry.queue[0].id, "b");
        industry.move_up(1);
        assert_eq!(industry.queue[0].id, "a");
    }

    #[test]
    fn looks_up_industry_cost_from_data() {
        let (data, registry) = load_game_data(PathBuf::from("assets/data")).expect("load data");
        let cost = industry_cost(&data, &registry, &BuildKind::Surface, "factory");
        assert!(cost.is_some());
    }
}
