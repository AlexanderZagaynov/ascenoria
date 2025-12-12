use crate::data::{GameData, LocalizedText, ResearchGraph, VictoryRules};

pub fn localized(text: &str) -> LocalizedText {
    LocalizedText {
        en: text.to_string(),
        ru: text.to_string(),
    }
}

pub fn base_game_data() -> GameData {
    GameData::new(
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ResearchGraph::default(),
        Vec::new(),
        VictoryRules::default(),
    )
}
