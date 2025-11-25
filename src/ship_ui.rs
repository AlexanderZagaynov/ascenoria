use bevy::prelude::Resource;

use crate::data::{GameData, HullClass, Language, LocalizedEntity};

/// Tracks available hulls and the current selection by identifier.
#[derive(Debug, Clone, Resource)]
pub struct HullSelection {
    hulls: Vec<HullClass>,
    selected: Option<usize>,
}

impl HullSelection {
    /// Build a selection list from loaded game data.
    pub fn from_game_data(data: &GameData) -> Self {
        Self::from_hulls(data.hull_classes().to_vec())
    }

    /// Build a selection list from provided hulls.
    pub fn from_hulls(hulls: Vec<HullClass>) -> Self {
        let selected = if hulls.is_empty() { None } else { Some(0) };
        Self { hulls, selected }
    }

    /// Move the selection to the next hull (wrapping).
    pub fn next(&mut self) {
        if let Some(idx) = self.selected {
            let next = (idx + 1) % self.hulls.len();
            self.selected = Some(next);
        }
    }

    /// Move the selection to the previous hull (wrapping).
    pub fn prev(&mut self) {
        if let Some(idx) = self.selected {
            let len = self.hulls.len();
            let prev = if idx == 0 { len - 1 } else { idx - 1 };
            self.selected = Some(prev);
        }
    }

    /// Get the currently selected hull id, if any.
    pub fn selected_id(&self) -> Option<&str> {
        self.selected
            .and_then(|idx| self.hulls.get(idx))
            .map(|hull| hull.id.as_str())
    }

    /// Render the hull list with the current selection highlighted.
    pub fn render(&self, language: Language) -> String {
        let mut lines = Vec::new();
        lines.push("Hull selection:".to_string());

        for (idx, hull) in self.hulls.iter().enumerate() {
            let marker = if Some(idx) == self.selected { ">" } else { " " };
            lines.push(format!(
                "{marker} {name} — {desc} (max items: {max})",
                name = hull.name(language),
                desc = hull.description(language),
                max = hull.max_items
            ));
        }

        if self.hulls.is_empty() {
            lines.push("No hulls available".to_string());
        }

        lines.join("\n")
    }
}
