//! Type definitions for the planet view screen.

use crate::planet_data::{BuildingType, PlanetSurface};
use bevy::prelude::*;

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
    pub selected_building: Option<BuildingType>,
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
pub enum UIAction {
    EndTurn,
    SelectBuilding(BuildingType),
    Quit,
}

#[derive(Component)]
pub struct VictoryMessage;

#[derive(Debug, Event, bevy::prelude::Message)]
pub struct TileUpdateEvent {
    pub x: usize,
    pub y: usize,
}

/// Colors for the planet view UI - inspired by Ascendancy's planet screen.
pub mod colors {}
