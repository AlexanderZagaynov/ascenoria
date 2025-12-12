use bevy::prelude::*;
use crate::data::Language;

/// Current language selection for UI rendering.
#[derive(Resource, Default)]
pub struct LocalizationSettings {
    pub language: Language,
}

impl LocalizationSettings {
    pub fn toggle(&mut self) {
        self.language = match self.language {
            Language::En => Language::Ru,
            Language::Ru => Language::En,
        };
    }
}
