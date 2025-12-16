//! GameData: the main data container for all loaded game content.
//!
//! This struct holds all game content loaded from TOML files: species, planets,
//! ships, technologies, etc. It's stored as a Bevy `Resource` so ECS systems
//! can query it.

mod accessors;
mod definition;
mod logic;
mod mutators;

pub use definition::GameData;
