//! Data loading functions for RON game data files.
//!
//! # Module Structure
//! - [`ron_loader`] - Low-level RON parsing helpers
//! - [`wrappers`] - Intermediate deserialization types
//! - [`root`] - Main `load_game_data()` entry point
//!
//! # Data Files
//! Loads the following RON files from `assets/data/`:
//! - `surface_cell_types.ron` - Terrain types
//! - `surface_buildings.ron` - Building definitions
//! - `technologies.ron` - Research tree
//! - `victory_conditions.ron` - Win/lose conditions
//! - `scenarios.ron` - Game scenarios

mod ron_loader;
mod wrappers;
mod root;

pub use root::load_game_data;
