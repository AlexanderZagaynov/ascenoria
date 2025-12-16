//! Ascenoria game library.
//!
//! This library exposes the core game data types, loading functions, and ECS systems
//! for the Ascenoria 4X strategy game.
//!
//! # Module Overview
//!
//! ## Data Layer
//! - [`data_types`] - RON data structures, loaders, and validation
//! - [`game_data`] - Bevy plugin for loading game data at startup
//!
//! ## Game Logic
//! - [`planet_data`] - Planet surface generation and tile types
//!
//! ## Presentation Layer
//! - [`main_menu`] - Main menu screen and game state machine
//! - [`planet_view`] - Planet surface management screen (3D + UI)

pub mod data_types;
pub mod game_data;
pub mod main_menu;
pub mod planet_data;
pub mod planet_view;
