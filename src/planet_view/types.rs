//! Type definitions for the planet view screen.

use crate::planet_data::{BuildingType, PlanetSurface};
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    Building(BuildingType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionProject {
    pub project_type: ProjectType,
    pub total_cost: u32,
    pub progress: u32,
    pub target_tile_index: usize,
}

#[derive(Resource, Default)]
pub struct PlanetViewState {
    pub surface: Option<PlanetSurface>,
    pub turn: u32,
    pub food: u32,
    pub housing: u32,
    pub production: u32,
    pub science: u32,
    pub research_progress: u32,
    pub terraforming_unlocked: bool,
    pub victory: bool,
    pub production_queue: VecDeque<ProductionProject>,
    pub build_menu_open: bool,
    pub build_menu_target_tile: Option<usize>,
}

#[derive(Component)]
pub struct PlanetViewRoot;

#[derive(Component)]
pub struct PlanetView3D;

#[derive(Component)]
pub struct TileEntity {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct BuildingEntity;

#[derive(Component)]
pub struct PlanetViewCursor;

#[derive(Component)]
pub enum UIAction {
    EndTurn,
    // SelectBuilding(BuildingType), // Removed
    Quit,
    // OpenBuildMenu, // Removed
}

#[derive(Component)]
pub struct VictoryMessage;

#[derive(Debug, Event, bevy::prelude::Message)]
pub struct TileUpdateEvent {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource, Default)]
pub struct PlanetViewAssets {
    pub large_plate_mesh: Handle<Mesh>,
    pub small_diamond_mesh: Handle<Mesh>,
    // pub white_mat: Handle<StandardMaterial>,
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
