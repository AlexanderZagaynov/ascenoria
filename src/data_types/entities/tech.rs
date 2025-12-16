//! Technology (research) data structures.
//!
//! Technologies form a research tree that players progress through
//! by spending science points.

use serde::Deserialize;

/// A technology that can be researched.
///
/// # RON Example
/// ```ron
/// (
///     id: "tech_advanced_farming",
///     name_en: "Advanced Farming",
///     science_cost: 100,
/// )
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Technology {
    /// Unique identifier (e.g., "tech_advanced_farming").
    pub id: String,
    /// English display name.
    pub name_en: String,
    /// Science points required to research.
    pub science_cost: i32,
}
