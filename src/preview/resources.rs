use bevy::prelude::*;
use crate::galaxy_data::Galaxy;
use crate::industry::PlanetIndustry;
use crate::planet_data::GeneratedPlanet;
use crate::research::ResearchState;
use crate::victory::{DominationConfig, VictoryState};

/// Holds a generated planet for debug visualization.
#[derive(Resource, Default)]
pub struct PlanetPreview {
    pub planet: Option<GeneratedPlanet>,
}

/// Holds a generated galaxy snapshot.
#[derive(Resource)]
pub struct GalaxyPreview {
    pub galaxy: Galaxy,
}

impl Default for GalaxyPreview {
    fn default() -> Self {
        Self {
            galaxy: Galaxy {
                systems: Vec::new(),
            },
        }
    }
}

/// Placeholder industry/build queue preview.
#[derive(Resource, Default)]
pub struct IndustryPreview {
    pub industry: PlanetIndustry,
}

/// Tracks domination victory progress.
#[derive(Resource)]
pub struct VictoryPreview {
    pub state: VictoryState,
}

impl Default for VictoryPreview {
    fn default() -> Self {
        Self {
            state: VictoryState::new(0, DominationConfig::default()),
        }
    }
}

/// Research progress and selection.
#[derive(Resource)]
pub struct ResearchPreview {
    pub state: ResearchState,
}

impl Default for ResearchPreview {
    fn default() -> Self {
        Self {
            state: ResearchState::new(1),
        }
    }
}
