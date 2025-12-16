//! Planet View UI modules.
//!
//! This module organizes all UI components for the planet surface screen:
//!
//! - [`panels`] - Left and right info panels (production queue, yields, etc.)
//! - [`top_bar`] - Top navigation bar with planet info and back button
//! - [`build_menu`] - Building selection modal dialog

pub mod panels;
pub mod top_bar;
pub mod build_menu;


pub use panels::{spawn_left_panel, spawn_right_panel};
pub use top_bar::spawn_top_bar;
