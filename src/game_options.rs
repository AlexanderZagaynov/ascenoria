//! Species selection screen for new game setup, inspired by Ascendancy.
//!
//! This module is split into smaller submodules to keep file sizes short:
//! - `types`: ECS components/resources
//! - `ui`: UI building/spawn helpers (and `ui::colors`)
//! - `impl`: plugin + systems (kept separate from data structures)

#[path = "game_options/types.rs"]
pub mod types;

#[path = "game_options/ui.rs"]
pub mod ui;

#[path = "game_options/impl.rs"]
mod r#impl;

pub use r#impl::GameOptionsPlugin;
pub use types::NewGameSettings;
