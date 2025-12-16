//! Strongly-typed ID types for game entities.
//!
//! Each entity type (building, technology, etc.) has its own ID type
//! to prevent accidentally mixing IDs from different entity types.
//!
//! # Example
//! ```ignore
//! let building_id: SurfaceBuildingId = "building_farm_1".into();
//! let tech_id: TechnologyId = "tech_agriculture".into();
//! // These are different types and cannot be confused!
//! ```

/// Macro to define a strongly-typed ID wrapper.
///
/// Creates a newtype struct around `String` with conversions
/// from `&str` and `String`, plus an `as_str()` accessor.
macro_rules! define_id_type {
    ($name:ident) => {
        /// Strongly-typed identifier for this entity type.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub String);

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            /// Get the underlying string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

// ID type for surface cell types (terrain).
define_id_type!(SurfaceCellTypeId);
// ID type for surface buildings.
define_id_type!(SurfaceBuildingId);
// ID type for technologies in the research tree.
define_id_type!(TechnologyId);
// ID type for victory conditions.
define_id_type!(VictoryConditionId);
// ID type for game scenarios.
define_id_type!(ScenarioId);
