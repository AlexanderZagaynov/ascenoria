//! Localization types and traits for game data.
//!
//! This file defines the language enum, localized text wrapper, and traits
//! that allow entities to expose translated names and descriptions.
//! It does NOT load data or define game entities.

use serde::Deserialize;

/// Supported UI languages for localized strings.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    /// English localization.
    En,
    /// Russian localization.
    Ru,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

/// Localized text wrapper with English and Russian values.
#[derive(Debug, Clone, Deserialize)]
pub struct LocalizedText {
    /// English string.
    pub en: String,
    /// Russian string.
    pub ru: String,
}

impl LocalizedText {
    /// Resolve the text in the requested language.
    pub fn get(&self, language: Language) -> &str {
        match language {
            Language::En => &self.en,
            Language::Ru => &self.ru,
        }
    }
}

/// Trait for entities that expose localized name and description fields.
pub trait LocalizedEntity {
    /// Return the raw localized name fields.
    fn name_text(&self) -> &LocalizedText;
    /// Return the raw localized description fields.
    fn description_text(&self) -> &LocalizedText;
}

/// Trait for entities with a stable identifier.
pub trait HasId {
    /// Borrow the identifier as a string slice.
    fn id(&self) -> &str;
}

/// Trait for entities that expose a localized name.
pub trait NamedEntity: LocalizedEntity {
    /// Resolve the localized name.
    fn name(&self, language: Language) -> &str {
        self.name_text().get(language)
    }
}

/// Trait for entities that expose a localized description.
pub trait HasDescription: LocalizedEntity {
    /// Resolve the localized description.
    fn description(&self, language: Language) -> &str {
        self.description_text().get(language)
    }
}

impl<T: LocalizedEntity> NamedEntity for T {}
impl<T: LocalizedEntity> HasDescription for T {}

/// Macro to implement LocalizedEntity for types with `name` and `description` fields.
#[macro_export]
macro_rules! impl_localized_entity {
    ($type:ty) => {
        impl $crate::data_types::localization::LocalizedEntity for $type {
            fn name_text(&self) -> &$crate::data_types::localization::LocalizedText {
                &self.name
            }

            fn description_text(&self) -> &$crate::data_types::localization::LocalizedText {
                &self.description
            }
        }
    };
}

/// Macro to implement HasId for types with an `id` field.
#[macro_export]
macro_rules! impl_has_id {
    ($type:ty) => {
        impl $crate::data_types::localization::HasId for $type {
            fn id(&self) -> &str {
                &self.id
            }
        }
    };
}
