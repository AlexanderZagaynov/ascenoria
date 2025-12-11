//! TOML loading functions for game data.

mod merge;
mod migration;
mod mod_loading;
mod research_graph;
mod toml;
mod wrappers;

pub const DATA_SCHEMA_VERSION: u32 = 1;

pub use root::load_game_data;

mod root;
