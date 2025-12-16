//! Game entity data structures loaded from RON files.
//!
//! Each submodule defines the Rust types that correspond to RON data schemas.
//!
//! # Modules
//! - [`scenario`] - Game scenarios (starting conditions, galaxy settings)
//! - [`surface`] - Planet surface types and buildings
//! - [`tech`] - Technology/research tree entries
//! - [`victory`] - Victory and defeat conditions

mod scenario;
mod surface;
mod tech;
mod victory;

pub use scenario::{GenerationMode, Scenario};
pub use surface::{BuildableOn, SpecialBehavior, SurfaceBuilding, SurfaceCellType};
pub use tech::Technology;
pub use victory::{VictoryCondition, VictoryType};
