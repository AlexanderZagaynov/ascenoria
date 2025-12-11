use bevy::prelude::*;

/// Marker component for all species selection UI entities.
#[derive(Component)]
pub struct GameOptionsRoot;

/// Resource tracking current selection state.
#[derive(Resource)]
pub struct NewGameSettings {
    /// Index of currently selected species.
    pub selected_species_index: usize,
    /// Star density setting (0 = Sparse, 1 = Average, 2 = Dense).
    pub star_density: usize,
    /// Number of AI species (1-7).
    pub num_species: usize,
    /// Atmosphere type (0 = Neutral, 1 = Oxygen, etc.).
    pub atmosphere: usize,
    /// Player color index.
    pub player_color: usize,
    /// Galaxy seed for preview.
    pub galaxy_seed: u64,
}

impl Default for NewGameSettings {
    fn default() -> Self {
        Self {
            selected_species_index: 0,
            star_density: 1, // Average
            num_species: 5,  // Five Species
            atmosphere: 0,   // Neutral
            player_color: 0,
            galaxy_seed: rand::random(),
        }
    }
}

/// Marker for species list items.
#[derive(Component)]
pub struct SpeciesListItem {
    pub index: usize,
}

/// Marker for species name text.
#[derive(Component)]
pub struct SpeciesNameText;

/// Marker for species description text.
#[derive(Component)]
pub struct SpeciesDescriptionText;

/// Marker for galaxy info text.
#[derive(Component)]
pub struct GalaxyInfoText;

/// Marker for the scrollable viewport.
#[derive(Component)]
pub struct SpeciesListViewport;

/// Marker for the scrollbar thumb.
#[derive(Component)]
pub struct SpeciesListScrollThumb;

/// Marker for scroll buttons.
#[derive(Component)]
pub enum ScrollButton {
    Up,
    Down,
}

/// Settings buttons.
#[derive(Component, Debug, Clone, Copy)]
pub enum SettingsButton {
    StarDensity,
    NumSpecies,
    Atmosphere,
    PlayerColor(usize),
}

/// Begin game button.
#[derive(Component)]
pub struct BeginGameButton;
