//! Side panel UI components for the Planet View.
//!
//! - [`left`] - Production queue, population, and resource yields
//! - [`right`] - Orbital structure slots

mod left;
mod right;

pub use left::{spawn_left_panel, ProductionQueueList};
pub use right::spawn_right_panel;
