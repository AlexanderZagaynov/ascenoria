//! Strongly-typed ID newtypes for game entities.
//!
//! These provide compile-time safety when looking up entities by ID.
//! Each ID wraps a String and can be converted from &str or String.

/// Macro to define a strongly-typed ID newtype.
macro_rules! define_id_type {
    ($name:ident) => {
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
            /// Borrow the underlying identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

define_id_type!(SpeciesId);
define_id_type!(PlanetSizeId);
define_id_type!(PlanetSurfaceTypeId);
define_id_type!(PlanetaryItemId);
define_id_type!(PlanetaryProjectId);
define_id_type!(HullClassId);
define_id_type!(EngineId);
define_id_type!(WeaponId);
define_id_type!(ShieldId);
define_id_type!(ScannerId);
define_id_type!(SpecialModuleId);
define_id_type!(TechId);
define_id_type!(VictoryConditionId);
