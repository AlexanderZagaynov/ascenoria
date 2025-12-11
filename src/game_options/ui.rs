//! UI components for the game options screen.
//!
//! This module contains UI spawn functions split into smaller files:
//! - `colors`: Color constants
//! - `galaxy_panel`: Galaxy preview panel
//! - `species_panels`: Species info and list panels
//! - `controls`: Settings buttons and begin game button

pub mod colors;
mod controls;
mod galaxy_panel;
mod species_panels;

// Re-export colors module
pub use colors::*;

// Re-export spawn functions
pub use controls::{spawn_begin_button, spawn_setting_button, spawn_settings_buttons};
pub use galaxy_panel::spawn_galaxy_panel;
pub use species_panels::{
    spawn_species_info_panel, spawn_species_list_item, spawn_species_list_panel,
};
