//! Species selection screen for new game setup, inspired by Ascendancy.
//!
//! This module is split into smaller submodules to keep file sizes short:
//! - `types`: ECS components/resources
//! - `ui`: UI building/spawn helpers (and `ui::colors`)
//! - `systems`: Game logic and systems
//! - `plugin`: Plugin definition

#[path = "game_options/types.rs"]
pub mod types;

#[path = "game_options/ui.rs"]
pub mod ui;

#[path = "game_options/systems/mod.rs"]
pub mod systems;

#[path = "game_options/plugin.rs"]
pub mod plugin;

pub use plugin::GameOptionsPlugin;
pub use types::NewGameSettings;
