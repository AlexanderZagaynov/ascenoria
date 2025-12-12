//! Validation helpers for game data.
//!
//! Contains functions that check data integrity: non-negative values,
//! valid tech references, proper tile distributions, etc.

mod helpers;
mod tech;
mod game_data;

pub(crate) use helpers::{validate_non_negative, validate_positive, validate_non_negative_fields, validate_tile_distribution};
pub(crate) use tech::{validate_tech_reference, validate_tech_edges};
pub use tech::NO_TECH_REQUIREMENT;
pub(crate) use game_data::validate_game_data;
