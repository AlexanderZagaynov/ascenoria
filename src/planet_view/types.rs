//! Type definitions for the planet view screen.
//!
//! This module contains all the core data structures used by the planet view,
//! including the game state, UI components, and event types.
//!
//! # Architecture
//!
//! The planet view follows a data-driven architecture where:
//! - [`PlanetViewState`] holds all mutable game state for the current planet
//! - Component markers (e.g., [`TileEntity`], [`BuildingEntity`]) tag ECS entities
//! - [`TileUpdateEvent`] triggers visual updates when tile state changes
//! - [`PlanetViewAssets`] caches shared mesh/material handles for performance

use crate::planet_data::{BuildingType, PlanetSurface};
use bevy::prelude::*;
use std::collections::VecDeque;

/// The type of project that can be added to the production queue.
///
/// Currently only supports building construction, but could be extended
/// to include research projects, terraforming, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    /// Construct a building of the specified type.
    Building(BuildingType),
}

/// A project in the production queue awaiting completion.
///
/// Projects accumulate production points each turn until they reach
/// their total cost, at which point the building is placed on the target tile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionProject {
    /// What kind of project this is (building type, etc.).
    pub project_type: ProjectType,
    /// Total production points required to complete this project.
    pub total_cost: u32,
    /// Current accumulated production points.
    pub progress: u32,
    /// Index into the surface tiles array where the building will be placed.
    pub target_tile_index: usize,
}

/// Central state resource for the planet view screen.
///
/// This resource holds all the mutable game state for the currently viewed planet,
/// including resource totals, the surface grid, and the production queue.
///
/// # Lifecycle
///
/// - Created with defaults when entering `GameState::PlanetView`
/// - Populated by `setup::setup_planet_view` with generated planet data
/// - Modified by `systems::end_turn` each game turn
/// - Reset when leaving the planet view
#[derive(Resource, Default)]
pub struct PlanetViewState {
    /// The planet's surface grid containing tiles and buildings.
    pub surface: Option<PlanetSurface>,
    /// Current game turn number (starts at 0, incremented each End Turn).
    pub turn: u32,
    /// Accumulated food resource (sum of all building yields).
    pub food: u32,
    /// Accumulated housing capacity (sum of all building yields).
    pub housing: u32,
    /// Accumulated production points (used to build structures).
    pub production: u32,
    /// Accumulated science points (used for research).
    pub science: u32,
    /// Progress toward the current research goal (0-100).
    pub research_progress: u32,
    /// Whether terraforming technology has been unlocked.
    pub terraforming_unlocked: bool,
    /// Whether the victory condition has been achieved.
    pub victory: bool,
    /// Queue of buildings awaiting construction, processed FIFO.
    pub production_queue: VecDeque<ProductionProject>,
    /// Whether the build menu modal is currently open.
    pub build_menu_open: bool,
    /// The tile index where the next building will be placed (when menu is open).
    pub build_menu_target_tile: Option<usize>,
}

/// Marker component for UI entities that belong to the planet view.
///
/// All 2D UI elements spawned for the planet view should have this component
/// so they can be cleaned up when leaving the screen.
#[derive(Component)]
pub struct PlanetViewRoot;

/// Marker component for 3D entities that belong to the planet view.
///
/// All 3D meshes (tiles, buildings, cursor) should have this component
/// so they can be cleaned up when leaving the screen.
#[derive(Component)]
pub struct PlanetView3D;

/// Component attached to tile mesh entities, storing their grid position.
///
/// Used by the interaction system to identify which tile was clicked
/// and by the visual update system to find the correct entity to modify.
#[derive(Component)]
pub struct TileEntity {
    /// X coordinate in the surface grid (0 = leftmost column).
    pub x: usize,
    /// Y coordinate in the surface grid (0 = topmost row).
    pub y: usize,
}

/// Marker component for building mesh entities.
///
/// Buildings are spawned as separate entities from tiles, positioned
/// slightly above the tile surface.
#[derive(Component)]
pub struct BuildingEntity;

/// Marker component for the hover cursor entity.
///
/// A semi-transparent overlay that follows the mouse and highlights
/// the currently hovered tile.
#[derive(Component)]
pub struct PlanetViewCursor;

/// Component attached to UI buttons to define their action when clicked.
///
/// The `systems::ui_action_system` reads this component to determine
/// what should happen when a button is pressed.
#[derive(Component)]
pub enum UIAction {
    /// Advance the game by one turn, processing yields and production.
    EndTurn,
    /// Return to the main menu.
    Quit,
}

/// Marker component for the victory message overlay.
///
/// This UI element is hidden by default and shown when `PlanetViewState::victory`
/// becomes true.
#[derive(Component)]
pub struct VictoryMessage;

/// Event fired when a tile's visual representation needs to be updated.
///
/// This event triggers `systems::update_visuals_system` to refresh the tile's
/// mesh, material, and any associated building entity.
#[derive(Debug, Event, bevy::prelude::Message)]
pub struct TileUpdateEvent {
    /// X coordinate of the tile that changed.
    pub x: usize,
    /// Y coordinate of the tile that changed.
    pub y: usize,
}

/// Cached mesh and material handles for the planet view.
///
/// These assets are created once during setup and reused across all tiles
/// to avoid duplicating GPU resources.
#[derive(Resource, Default)]
pub struct PlanetViewAssets {
    /// Mesh for connected tiles (large flat plate).
    pub large_plate_mesh: Handle<Mesh>,
    /// Mesh for disconnected tiles (small diamond shape).
    pub small_diamond_mesh: Handle<Mesh>,
    /// Material for black (unbuildable) tiles.
    pub black_mat: Handle<StandardMaterial>,
}

/// Colors for the planet view UI - inspired by Ascendancy's planet screen.
pub mod colors {
    use bevy::prelude::Color;

    pub const PANEL_BG: Color = Color::srgb(0.1, 0.1, 0.2);
    pub const BORDER: Color = Color::srgb(0.5, 0.5, 0.7);
    pub const HEADER_TEXT: Color = Color::srgb(0.9, 0.9, 1.0);
    pub const TEXT: Color = Color::srgb(0.8, 0.8, 0.8);
    // pub const VALUE_TEXT: Color = Color::srgb(1.0, 1.0, 0.8);
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.3);
    // pub const THUMBNAIL_SELECTED: Color = Color::srgb(1.0, 1.0, 0.0);
    // pub const THUMBNAIL_NORMAL: Color = Color::srgb(0.5, 0.5, 0.5);
    // pub const TILE_WHITE: Color = Color::WHITE;
}
